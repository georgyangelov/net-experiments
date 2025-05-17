use std::{env, io};
use std::net::SocketAddr;
use bytes::BytesMut;
use net_experiments::net::proto;
use net_experiments::net::proto::packet::Data;
use prost::Message;
use s2n_quic::Client;
use s2n_quic::client::Connect;
use s2n_quic::stream::{BidirectionalStream, ReceiveStream, SendStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::JoinSet;

pub static CERT_PEM: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cert.pem"
));

pub static KEY_PEM: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/key.pem"
));

#[tokio::main]
pub async fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_usage()
    }

    if args[1] == "server" {
        server().await;
    } else if args[1] == "client" {
        client().await.expect("client error")
    } else {
        print_usage()
    }
}

fn print_usage() -> ! {
    let args: Vec<String> = env::args().collect();
    let bin = &args[0];

    println!("Usage: {bin} <server|client>");
    std::process::exit(-1);
}

async fn server() {
    let addr = "127.0.0.1:5555";

    println!("Listening on {addr}");

    let mut server = s2n_quic::Server::builder()
        .with_tls((CERT_PEM, KEY_PEM)).expect("could not configure tls")
        .with_io("127.0.0.1:5555").expect("could not configure io")
        .start().expect("could not start QUIC server");

    while let Some(mut connection) = server.accept().await {
        // spawn a new task for the connection
        tokio::spawn(async move {
            let (receive_sender, receive_receiver) = tokio::sync::mpsc::channel(4);
            let (send_sender, send_receiver) = tokio::sync::mpsc::channel(4);

            println!("Connection accepted from {:?}", connection.remote_addr());

            if let Ok(Some(stream)) = connection.accept_bidirectional_stream().await {
                let handle = tokio::spawn(async move {
                    println!("Stream opened from {:?}", stream.connection().remote_addr());

                    handle_stream(stream, receive_sender, send_receiver).await;
                });

                handle_server(send_sender, receive_receiver).await;

                handle.await.expect("error in server handler");
            }
        });
    }
}

async fn handle_stream(
    stream: BidirectionalStream,
    receive_sender: tokio::sync::mpsc::Sender<Data>,
    mut send_receiver: tokio::sync::mpsc::Receiver<Data>,
) {
    let mut read_buf = BytesMut::with_capacity(1024);
    let mut send_buf = BytesMut::with_capacity(1024);
    let mut scope = JoinSet::new();

    let (mut recv_stream, mut send_stream) = stream.split();

    scope.spawn(async move {
        loop {
            let Ok(data) = read_packet(&mut recv_stream, &mut read_buf).await else {
                // TODO: Better check
                break;
            };

            // TODO: Error handling, what if the receiver channel is closed?
            let result = receive_sender.send(data).await;
            if let Err(_) = result {
                break;
            }
        }
    });

    scope.spawn(async move {
        while let Some(data) = send_receiver.recv().await {
            match write_packet(&mut send_stream, data, &mut send_buf).await {
                Ok(_) => {}
                Err(_err) => {
                    // TODO: Better check
                    break;
                }
            }
        }

        match send_stream.close().await {
            Ok(_) => {}
            Err(_) => {
                // TODO: Better error handling here
            }
        }
    });

    scope.join_all().await;

    println!("connection closed");
}

async fn handle_server(
    send_sender: tokio::sync::mpsc::Sender<Data>,
    mut receive_receiver: tokio::sync::mpsc::Receiver<Data>,
) {
    let packet = receive_receiver.recv().await
        .expect("could not receive request");

    println!("Received '{packet:?}'");

    match packet {
        Data::Request(req) => {
            let mut text = req.text;

            text.push_str("!?!");

            send_sender.send(Data::Response(proto::Response { text })).await
                .expect("could not send response");

            println!("Sent response");
        }
        Data::Response(_) => panic!("invalid packet type Response")
    }
}

async fn client() -> io::Result<()> {
    let client = Client::builder()
        .with_tls(CERT_PEM).expect("could not configure TLS")
        .with_io("0.0.0.0:58585").expect("could not configure IO")
        .start().expect("could not start quic client");

    let addr: SocketAddr = "127.0.0.1:5555".parse()
        .expect("could not parse server address");
    let connect = Connect::new(addr)
        .with_server_name("localhost");

    let mut connection = client.connect(connect).await
        .expect("could not connect to server");

    // ensure the connection doesn't time out with inactivity
    connection.keep_alive(true)?;

    let stream = connection.open_bidirectional_stream().await?;

    let (receive_sender, receive_receiver) = tokio::sync::mpsc::channel(4);
    let (send_sender, send_receiver) = tokio::sync::mpsc::channel(4);

    let handle = tokio::spawn(async move {
        handle_stream(stream, receive_sender, send_receiver).await;
    });

    handle_client(send_sender, receive_receiver).await;

    handle.await.expect("error in client handler");

    Ok(())
}

async fn handle_client(
    send_sender: tokio::sync::mpsc::Sender<Data>,
    mut receive_receiver: tokio::sync::mpsc::Receiver<Data>,
) {
    let text = String::from("Hello world");
    send_sender.send(Data::Request(proto::Request { text: text.clone() })).await
        .expect("could not send request");

    let response = receive_receiver.recv().await
        .expect("could not read response");

    println!("Received response {response:?}");
}

// TODO: Result
async fn read_packet(stream: &mut ReceiveStream, buf: &mut BytesMut) -> io::Result<Data> {
    let mut header = [0u8; 8];

    stream.read_exact(&mut header).await?;

    let packet_len = u64::from_le_bytes(header) as usize;

    buf.resize(packet_len, 0);
    stream.read_exact(&mut buf[..packet_len]).await?;

    let packet = proto::Packet::decode(buf)
        .expect("could not decode packet from stream");

    match packet {
        proto::Packet { data: Some(data) } => Ok(data),
        _ => panic!("invalid packet")
    }
}

async fn write_packet(stream: &mut SendStream, data: Data, buf: &mut BytesMut) -> io::Result<()> {
    let packet = proto::Packet {
        data: Some(data)
    };

    let header = (packet.encoded_len() as u64).to_le_bytes();

    packet.encode(buf)
        .expect("could not encode packet to a buffer");

    stream.write_all(&header).await?;
    stream.write_all(&buf).await
}