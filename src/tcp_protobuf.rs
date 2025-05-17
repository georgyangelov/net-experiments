use std::{env, io, thread};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use net_experiments::net::proto;
use net_experiments::net::proto::packet::Data;
use prost::Message;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_usage()
    }

    if args[1] == "server" {
        server()
    } else if args[1] == "client" {
        client().expect("client error")
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

fn server() {
    let addr = "127.0.0.1:5555";
    let listener = TcpListener::bind(addr)
        .expect("could not start listening");

    println!("Listening on {addr}");

    thread::scope(|s| {
        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    s.spawn(move || handle_server_connection(stream, addr));
                },
                Err(err) => println!("Failed to accept connection from: {err:?}")
            }
        }
    })
}

fn handle_server_connection(mut stream: TcpStream, remote_addr: SocketAddr) {
    println!("Connection from {remote_addr}");

    let packet = read_packet(&mut stream);
    println!("Received '{packet:?}'");

    match packet {
        Data::Request(req) => {
            let mut text = req.text;

            text.push_str("!?!");

            write_packet(&mut stream, Data::Response(proto::Response { text }))
                .expect("could not send response");

            println!("Sent response");
        }
        Data::Response(_) => panic!("invalid packet type Response")
    }
}

fn client() -> io::Result<()> {
    let mut conn = TcpStream::connect("127.0.0.1:5555")?;

    let text = String::from("Hello world");
    write_packet(&mut conn, Data::Request(proto::Request { text: text.clone() }))?;
    println!("Sent '{text}'");

    let response = read_packet(&mut conn);
    println!("Received response {response:?}");

    Ok(())
}

fn read_packet(stream: &mut TcpStream) -> Data {
    let mut header = [0u8; 8];
    let mut buf = bytes::BytesMut::new();

    stream.read_exact(&mut header)
        .expect("could not read str len");

    let packet_len = u64::from_le_bytes(header) as usize;

    buf.resize(packet_len, 0);
    stream.read_exact(&mut buf[..packet_len])
        .expect("could not read the packet data");

    let packet = proto::Packet::decode(buf)
        .expect("could not decode packet from stream");

    match packet {
        proto::Packet { data: Some(data) } => data,
        _ => panic!("invalid packet")
    }
}

fn write_packet(stream: &mut TcpStream, data: Data) -> io::Result<()> {
    let packet = proto::Packet {
        data: Some(data)
    };

    let header = (packet.encoded_len() as u64).to_le_bytes();

    let mut buf = bytes::BytesMut::with_capacity(header.len() + packet.encoded_len());
    packet.encode(&mut buf)
        .expect("could not encode packet to a buffer");

    stream.write_all(&header)?;
    stream.write_all(&buf)
}