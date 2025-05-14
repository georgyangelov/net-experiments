// use std::{env, io, thread};
// use std::io::{Read, Write};
// use std::net::{SocketAddr, TcpListener, TcpStream};
// use flatbuffers::FlatBufferBuilder;
// use net_experiments::{root_as_packet, Message, PacketArgs, Request, RequestArgs, Response, ResponseArgs};
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
// fn server() {
//     let addr = "127.0.0.1:5555";
//     let listener = TcpListener::bind(addr)
//         .expect("could not start listening");
//
//     println!("Listening on {addr}");
//
//     thread::scope(|s| {
//         loop {
//             match listener.accept() {
//                 Ok((stream, addr)) => {
//                     s.spawn(move || handle_server_connection(stream, addr));
//                 },
//                 Err(err) => println!("Failed to accept connection from: {err:?}")
//             }
//         }
//     })
// }
//
// fn handle_server_connection(mut stream: TcpStream, remote_addr: SocketAddr) {
//     println!("Connection from {remote_addr}");
//
//     let mut buffer = Vec::with_capacity(1024);
//
//     read_packet(&mut stream, &mut buffer);
//
//     let packet = root_as_packet(&buffer)
//         .expect("could not decode packet from stream");
//
//     println!("Received '{packet:?}'");
//
//     match packet.message_type() {
//         Message::Request => {
//             let req = packet.message_as_request().unwrap();
//
//             let mut text = String::from(req.text().unwrap());
//             text.push_str("!?!");
//
//             write_packet(&mut stream, |b| {
//                 let s = b.create_string(&text);
//
//                 let req = Response::create(b, &ResponseArgs {
//                     text: Some(s)
//                 });
//
//                 PacketArgs {
//                     message_type: Message::Response,
//                     message: Some(req.as_union_value()),
//                 }
//             }).expect("could not send response");
//
//             println!("Sent response");
//         }
//         Message::Response => panic!("invalid packet type Response"),
//         _ => panic!("invalid packet type")
//     }
// }
//
// fn client() -> io::Result<()> {
//     let mut conn = TcpStream::connect("127.0.0.1:5555")?;
//
//     let mut buffer = Vec::with_capacity(1024);
//
//     let text = String::from("Hello world");
//     write_packet(&mut conn, |b| {
//         let s = b.create_string(&text);
//
//         let req = Request::create(b, &RequestArgs {
//             text: Some(s)
//         });
//
//         PacketArgs {
//             message_type: Message::Request,
//             message: Some(req.as_union_value()),
//         }
//     })?;
//     println!("Sent '{text}'");
//
//     read_packet(&mut conn, &mut buffer);
//     let packet = root_as_packet(&buffer)
//         .expect("could not decode packet from stream");
//
//     println!("Received response {packet:?}");
//
//     Ok(())
// }
//
// fn read_packet(stream: &mut TcpStream, buf: &mut Vec<u8>) {
//     let mut header = [0u8; 8];
//
//     stream.read_exact(&mut header)
//         .expect("could not read str len");
//
//     let packet_len = u64::from_le_bytes(header) as usize;
//
//     buf.resize(packet_len, 0);
//     stream.read_exact(buf)
//         .expect("could not read the packet data");
//
//     // let packet = root_as_packet(&buf)
//     //     .expect("could not decode packet from stream");
//     //
//     // packet
//
//     // let packet = proto::Packet::decode(buf)
//     //     .expect("could not decode packet from stream");
//     //
//     // match packet {
//     //     proto::Packet { data: Some(data) } => data,
//     //     _ => panic!("invalid packet")
//     // }
// }
//
// fn write_packet(stream: &mut TcpStream, data: impl FnOnce(&mut FlatBufferBuilder) -> PacketArgs) -> io::Result<()> {
//     let mut builder = FlatBufferBuilder::with_capacity(1024);
//
//     let data = data(&mut builder);
//
//     let packet = net_experiments::Packet::create(&mut builder, &data);
//
//     builder.finish(packet, None);
//
//     let binary_data = builder.finished_data();
//     let header = (binary_data.len() as u64).to_le_bytes();
//
//     stream.write_all(&header)?;
//     stream.write_all(&binary_data)
// }