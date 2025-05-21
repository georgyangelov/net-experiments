#![allow(dead_code)]

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
    lmdb::run();
}