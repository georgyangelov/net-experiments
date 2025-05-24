#![allow(dead_code)]

use std::thread;
use std::time::Duration;
use net_experiments::fs_event_lib::{config_flags, FSEventConfig, FSEvents};

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

    let config = FSEventConfig{
        paths_to_watch: vec!["/Users/stormbreaker/dev/net-experiments".into()],
        latency: Duration::from_millis(1000),
        since_event_id: None,
        flags: config_flags::FILE_EVENTS | config_flags::IGNORE_SELF
    };

    let fs_events = FSEvents::new(config, |e| {
        println!("Event: {:?}", e.items);
    }).expect("could not configure FSEvents listener");

    let listener = fs_events.start_listening().expect("could not start listening");
    println!("Started listening");

    thread::sleep(Duration::from_secs(5));

    listener.stop_listening();
    println!("Stopped listening");
}