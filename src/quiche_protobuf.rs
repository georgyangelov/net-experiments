// use std::{env, io, thread};
// use std::collections::HashMap;
// use std::io::{Read, Write};
// use std::net::UdpSocket;
// use mio::Interest;
// use net_experiments::net::proto;
// use net_experiments::net::proto::packet::Data;
// use prost::Message;
// use ring::rand::SystemRandom;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     if args.len() == 1 {
//         print_usage()
//     }
//
//     if args[1] == "server" {
//         server()
//     } else if args[1] == "client" {
//         client().expect("client error")
//     } else {
//         print_usage()
//     }
// }
//
// fn print_usage() -> ! {
//     let args: Vec<String> = env::args().collect();
//     let bin = &args[0];
//
//     println!("Usage: {bin} <server|client>");
//     std::process::exit(-1);
// }
//
// const MAX_DATAGRAM_SIZE: usize = 1350;
//
// struct PartialResponse {
//     body: Vec<u8>,
//
//     written: usize,
// }
//
// struct Client {
//     conn: quiche::Connection,
//
//     partial_responses: HashMap<u64, PartialResponse>,
// }
//
// type ClientMap = HashMap<quiche::ConnectionId<'static>, Client>;
//
// const UDP_SOCKET_TOKEN: mio::Token = mio::Token(0);
//
// fn server() {
//     let addr = "127.0.0.1:5555";
//     let mut socket = mio::net::UdpSocket::bind(addr.parse().unwrap())
//         .expect("could not bind to UDP socket");
//
//     let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION)?;
//
//     config.set_application_protos(&[b"net-experiments"])
//         .expect("could not configure app protocols");
//
//     config.discover_pmtu(true);
//     config.set_disable_active_migration(true);
//     config.set_max_recv_udp_payload_size(MAX_DATAGRAM_SIZE);
//     config.set_max_send_udp_payload_size(MAX_DATAGRAM_SIZE);
//
//     let rng = SystemRandom::new();
//     let conn_id_key = ring::hmac::Key::generate(ring::hmac::HMAC_SHA256, &rng)
//         .expect("could not create HMAC key");
//
//     let local_addr = socket.local_addr().unwrap();
//
//     let mut buf = [0u8; 65536];
//
//     println!("Listening on {addr}");
//
//     let mut poll = mio::Poll::new().expect("couldn't create mio pool");
//     let mut events = mio::Events::with_capacity(128);
//
//     poll.registry()
//         .register(&mut socket, UDP_SOCKET_TOKEN, Interest::READABLE)
//         .expect("could not register socket for mio");
//
//     thread::scope(|s| {
//         loop {
//             // TODO: Provide timeout - the shortest of the active connections
//             poll.poll(&mut events, None).expect("could not poll for events");
//
//             // Read until we can
//             'read: loop {
//                 let (len, from) = match socket.recv_from(&mut buf) {
//                     Ok(v) => v,
//                     Err(err) => {
//                         if err.kind() == io::ErrorKind::WouldBlock {
//                             break 'read;
//                         }
//
//                         panic!("recv_from failed {err:?}")
//                     }
//                 };
//                 let packet_buf = &mut buf[..len];
//
//                 // Parse the QUIC packet's header.
//                 let quic_header = match quiche::Header::from_slice(
//                     packet_buf,
//                     quiche::MAX_CONN_ID_LEN,
//                 ) {
//                     Ok(v) => v,
//                     Err(err) => {
//                         println!("could not parse header {err:?}");
//                         continue 'read;
//                     },
//                 };
//
//                 // let conn_id = ring::hmac::sign(&conn_id_seed)
//
//             }
//
//             // match listener.accept() {
//             //     Ok((stream, addr)) => {
//             //         s.spawn(move || handle_server_connection(stream, addr));
//             //     },
//             //     Err(err) => println!("Failed to accept connection from: {err:?}")
//             // }
//         }
//     })
// }
//
// fn handle_server_connection(mut stream: TcpStream, remote_addr: SocketAddr) {
//     println!("Connection from {remote_addr}");
//
//     let packet = read_packet(&mut stream);
//     println!("Received '{packet:?}'");
//
//     match packet {
//         Data::Request(req) => {
//             let mut text = req.text;
//
//             text.push_str("!?!");
//
//             write_packet(&mut stream, Data::Response(proto::Response { text }))
//                 .expect("could not send response");
//
//             println!("Sent response");
//         }
//         Data::Response(_) => panic!("invalid packet type Response")
//     }
// }
//
// fn client() -> io::Result<()> {
//     let mut conn = TcpStream::connect("127.0.0.1:5555")?;
//
//     let text = String::from("Hello world");
//     write_packet(&mut conn, Data::Request(proto::Request { text: text.clone() }))?;
//     println!("Sent '{text}'");
//
//     let response = read_packet(&mut conn);
//     println!("Received response {response:?}");
//
//     Ok(())
// }
//
// fn read_packet(stream: &mut TcpStream) -> Data {
//     let mut header = [0u8; 8];
//     let mut buf = bytes::BytesMut::new();
//
//     stream.read_exact(&mut header)
//         .expect("could not read str len");
//
//     let packet_len = u64::from_le_bytes(header) as usize;
//
//     buf.resize(packet_len, 0);
//     stream.read_exact(&mut buf[..packet_len])
//         .expect("could not read the packet data");
//
//     let packet = proto::Packet::decode(buf)
//         .expect("could not decode packet from stream");
//
//     match packet {
//         proto::Packet { data: Some(data) } => data,
//         _ => panic!("invalid packet")
//     }
// }
//
// fn write_packet(stream: &mut TcpStream, data: Data) -> io::Result<()> {
//     let packet = proto::Packet {
//         data: Some(data)
//     };
//
//     let header = (packet.encoded_len() as u64).to_le_bytes();
//
//     let mut buf = bytes::BytesMut::with_capacity(header.len() + packet.encoded_len());
//     packet.encode(&mut buf)
//         .expect("could not encode packet to a buffer");
//
//     stream.write_all(&header)?;
//     stream.write_all(&buf)
// }