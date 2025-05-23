#![allow(dead_code)]

mod chunk_and_hash_file;
mod s2n_quic_protobuf;
mod tcp_protobuf;
mod tcp_flatbuffers;
mod chunk_and_hash_parallel;
mod chunk_and_hash_parallel_bytes;
mod lmdb;

#[cxx::bridge]
mod ffi {
    #[derive(Debug)]
    enum FSEventStartResult {
        Done,
        Failed
    }

    extern "Rust" {
        type FSEventContext;
    }

    unsafe extern "C++" {
        include!("net-experiments/cpp/hello.h");

        type MacOSFSEventsMonitor;

        fn new_fs_events_monitor(
            ctx: Box<FSEventContext>,
            callback: fn(&mut FSEventContext, String),
        ) -> UniquePtr<MacOSFSEventsMonitor>;

        fn start(self: Pin<&mut MacOSFSEventsMonitor>) -> FSEventStartResult;
    }
}

struct FSEventContext {}

fn main() {
    // tcp_protobuf::run();
    // tcp_flatbuffers::run();
    // s2n_quic_protobuf::run();
    // chunk_and_hash_file::run();
    // chunk_and_hash_parallel::run();
    // chunk_and_hash_parallel_bytes::run();
    // lmdb::run();

    let context = Box::new(FSEventContext {});

    let mut client = ffi::new_fs_events_monitor(context, |ctx, msg| println!("{msg}"));
    let client = client.pin_mut();
    let result = client.start();

    println!("Result: {result:?}")
}

pub(crate) fn on_fs_change(log: String) {
    println!("FS Change: {log}")
}