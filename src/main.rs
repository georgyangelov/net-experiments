#![allow(dead_code)]

use net_experiments::fs_event_lib::FSEvents;

mod chunk_and_hash_file;
mod s2n_quic_protobuf;
mod tcp_protobuf;
mod tcp_flatbuffers;
mod chunk_and_hash_parallel;
mod chunk_and_hash_parallel_bytes;
mod lmdb;

fn main() {
    // tcp_protobuf::run();
    // tcp_flatbuffers::run();
    // s2n_quic_protobuf::run();
    // chunk_and_hash_file::run();
    // chunk_and_hash_parallel::run();
    // chunk_and_hash_parallel_bytes::run();
    // lmdb::run();

    let mut fs_events = FSEvents::new(Box::new(|e| {
        println!("Event: {:?}", e.items);
    }));

    fs_events.listen();

    // let test = String::from("test");
    // let test_ref = &test;
    //
    // let f = || { test_ref.clone() };
    //
    // testt(f);
    //
    // println!("{}", test);

    // let context = Box::new(FSEventContext {});
    //
    // let mut client = ffi::new_fs_events_monitor(context, |ctx, msg| println!("{msg}"));
    // let client = client.pin_mut();
    // let result = client.start();
    //
    // println!("Result: {result:?}")
}

fn testt<F>(f: F) where F: Fn() -> String {
    println!("{}", f());
}

pub(crate) fn on_fs_change(log: String) {
    println!("FS Change: {log}")
}