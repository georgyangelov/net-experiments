mod chunk_and_hash_file;
mod s2n_quic_protobuf;
mod tcp_protobuf;
mod tcp_flatbuffers;

fn main() {
    // tcp_protobuf::run();
    // tcp_flatbuffers::run();
    // s2n_quic_protobuf::run();
    chunk_and_hash_file::run();
}