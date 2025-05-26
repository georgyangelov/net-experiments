#![allow(dead_code)]

mod chunk_and_hash_file;
mod s2n_quic_protobuf;
mod tcp_protobuf;
mod tcp_flatbuffers;
mod chunk_and_hash_parallel;
mod chunk_and_hash_parallel_bytes;
mod lmdb;
mod fs_events;
mod scan_dir;
mod scan_dir_tree_queue;
mod scan_dir_tree_recursive;
mod scan_dir_tree_queue_unsafe;
mod scan_dir_tree_queue_unsafe_cell;

fn main() {
    // tcp_protobuf::run();
    // tcp_flatbuffers::run();
    // s2n_quic_protobuf::run();
    // chunk_and_hash_file::run();
    // chunk_and_hash_parallel::run();
    // chunk_and_hash_parallel_bytes::run();
    // lmdb::run();
    // fs_events::run();
    // scan_dir::run();
    // scan_dir_tree_queue::run();
    // scan_dir_tree_recursive::run();
    // scan_dir_tree_queue_unsafe::run();
    scan_dir_tree_queue_unsafe_cell::run();
}
