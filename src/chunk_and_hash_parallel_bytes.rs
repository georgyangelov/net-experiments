use std::fs::File;
use std::io::{ErrorKind, Read};
use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Instant;
use bytes::{Bytes, BytesMut};

pub fn run() {
    let now = Instant::now();

    let chunks = chunk_hash_file(
        "/Users/stormbreaker/Downloads/big-file.data",
        1*1024*1024,
        1*1024*1024,
    );

    let chunk_count = chunks.len();
    let elapsed = now.elapsed().as_secs_f32() * 1000f32;

    println!("Computed {chunk_count} chunks in {elapsed}ms")
    // println!("Computed {hash_hex} in {elapsed}ms")
}

fn chunk_hash_file(path: &str, chunk_size: usize, read_size: usize) -> Vec<blake3::Hash> {
    assert!(read_size <= chunk_size);
    assert_eq!(chunk_size % read_size, 0);
    let buffer_size = 1;

    println!("Chunk size: {chunk_size}");
    println!("Read size: {read_size}");
    println!("Buffer: {buffer_size}");

    let mut buf = BytesMut::new();
    let (send, recv) = std::sync::mpsc::sync_channel(buffer_size);

    let result = std::thread::scope(|s| {
        let buf_ref = &mut buf;
        s.spawn(move || chunker(path, read_size, send, buf_ref));

        hasher(recv, chunk_size)
    });

    let _ = buf.try_reclaim(1);
    println!("Total buffer capacity used: {}", buf.capacity());

    result
}

fn chunker(path: &str, read_chunk_size: usize, send: SyncSender<Bytes>, buf: &mut BytesMut) {
    let mut file = File::open(path).expect("could not open file");

    loop {
        buf.resize(read_chunk_size, 0u8);
        let mut buf = buf.split();

        let mut read = 0;

        while read < read_chunk_size {
            match file.read(&mut buf[read..read_chunk_size]) {
                Ok(n) => {
                    if n == 0 {
                        break
                    }

                    read += n
                },
                Err(err) => {
                    if err.kind() != ErrorKind::Interrupted {
                        panic!("could not read file: {err:?}")
                    }
                }
            };
        };

        if read < read_chunk_size {
            buf.truncate(read);
        }

        if let Err(_) = send.send(buf.freeze()) {
            // Receiver is closed, stop chunking
            break
        };

        if read < read_chunk_size {
            // We have EOF, we implicitly drop the Sender which would signal EOF for
            // the consumers
            break
        }
    }
}

fn hasher(recv: Receiver<Bytes>, hash_chunk_size: usize) -> Vec<blake3::Hash> {
    let mut hasher = blake3::Hasher::new();
    let mut hashes = Vec::new();

    let mut current_n = 0;

    while let Ok(chunk) = recv.recv() {
        current_n += chunk.len();

        hasher.update(&chunk);

        if current_n == hash_chunk_size {
            hashes.push(hasher.finalize());
            hasher.reset();
            current_n = 0;
        }
    }

    // Final chunk will be smaller
    if current_n > 0 {
        hashes.push(hasher.finalize());
        hasher.reset();
    }

    hashes
}
