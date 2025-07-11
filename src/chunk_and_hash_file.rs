use std::cmp::min;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::time::Instant;

pub fn run() {
    let now = Instant::now();

    // let hash = hash_file("/Users/stormbreaker/Downloads/big-file.data");
    // let hash_hex = hash.to_hex();

    let chunks = chunk_hash_file(
        "/Users/stormbreaker/Downloads/big-file.data",
        1*1024*1024,
        64*1024
    );

    let chunk_count = chunks.len();
    let elapsed = now.elapsed().as_secs_f32() * 1000f32;

    println!("Computed {chunk_count} chunks in {elapsed}ms")
    // println!("Computed {hash_hex} in {elapsed}ms")
}

fn hash_file(path: &str) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();

    let mut file = File::open(path)
        .expect("could not open file");

    let _len = std::io::copy(&mut file, &mut hasher)
        .expect("could not read and hash file");

    hasher.finalize()
}

fn chunk_hash_file(path: &str, chunk_size: usize, read_size: usize) -> Vec<blake3::Hash> {
    assert!(read_size <= chunk_size);
    assert_eq!(chunk_size % read_size, 0);

    let mut hasher = blake3::Hasher::new();
    let mut file = File::open(path).expect("could not open file");
    // let mut buf_reader = BufReader::new(file);

    let mut buf = Vec::new();
    buf.resize(chunk_size, 0u8);

    let mut hashes = Vec::new();

    loop {
        let mut read = 0;

        while read < chunk_size {
            let to_read_size = min(chunk_size - read, read_size);

            // Reading in smaller batches to balance hashing with IO
            // But it's still single-threaded
            match file.read(&mut buf[read..(read + to_read_size)]) {
                Ok(n) => {
                    if n == 0 {
                        break
                    }

                    hasher.update(&buf[read..(read+n)]);

                    read += n
                },
                Err(err) => {
                    if err.kind() != ErrorKind::Interrupted {
                        panic!("could not read file: {err:?}")
                    }
                }
            };
        };

        hashes.push(hasher.finalize());
        hasher.reset();

        if read < chunk_size {
            // We have EOF
            break
        }
    }

    hashes
}