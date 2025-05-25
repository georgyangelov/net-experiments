use std::collections::VecDeque;
use std::{fs, thread};
use std::path::{PathBuf};
use std::time::Duration;
use tokio::time::Instant;

pub fn run() {
    let now = Instant::now();

    let paths = read_dir_recursive("/Users/stormbreaker/dev-work".into());

    let elapsed = now.elapsed().as_secs_f32() * 1000f32;

    println!("Done in {elapsed}ms");

    loop {
        thread::sleep(Duration::from_secs(5));

        println!("Paths: {}", paths.len())
    }
}

struct NextDir {
    rel_path: PathBuf,
}

fn read_dir_recursive(root_path: PathBuf) -> Vec<PathBuf> {
    let mut next_dirs = VecDeque::new();
    let mut dir_count = 0;
    let mut file_count = 0;
    let mut paths = Vec::new();

    next_dirs.push_back(NextDir { rel_path: "".into() });

    while let Some(next) = next_dirs.pop_front() {
        let abs_path = root_path.join(&next.rel_path);
        let readdir = match fs::read_dir(&abs_path) {
            Ok(dir) => dir,
            Err(err) => {
                println!("Could not open dir {:?}: {:?}", &abs_path, err);
                continue;
            }
        };

        for entry in readdir {
            let entry = entry.expect("could not read dir");

            let name = entry.file_name();
            let typ = entry.file_type().expect("could not get file type");
            let rel_path = next.rel_path.join(name);

            paths.push(rel_path.clone());

            if typ.is_dir() {
                // println!("- {rel_path:?}/");
                dir_count += 1;

                next_dirs.push_back(NextDir { rel_path })
            } else if typ.is_file() {
                file_count += 1;

                // println!("- {rel_path:?}")
            } else {
                // println!("- {rel_path:?}??")
            }
        }
    }

    println!("Dirs: {dir_count}");
    println!("Files: {file_count}");

    paths
}