
#[allow(dead_code, unused_imports)]
#[path = "./types_generated.rs"]
mod flatbuf;
pub mod fs_event_lib;

pub use flatbuf::net::flatbuf::*;

pub mod net {
    pub mod proto {
        include!(concat!(env!("OUT_DIR"), "/net.proto.rs"));
    }
}
