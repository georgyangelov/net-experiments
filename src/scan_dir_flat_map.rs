use std::collections::{BTreeMap, VecDeque};
use std::{fs, io, thread};
use std::path::{PathBuf};
use std::time::Duration;
use tokio::time::Instant;

pub fn run() {
    let now = Instant::now();

    let root = scan_dir_recursively("/Users/stormbreaker/dev-work".into())
        .expect("could not scan dir");

    let elapsed = now.elapsed().as_secs_f32() * 1000f32;

    println!("Done in {elapsed}ms");

    loop {
        thread::sleep(Duration::from_secs(5));

        println!("Total objects: {:?}", root.entries.len())
    }
}

struct State {
    entries: BTreeMap<PathBuf, Entry>
}

enum Entry {
    Dir,
    File
}

fn scan_dir_recursively(root_path: PathBuf) -> io::Result<State> {
    let mut state = State { entries: BTreeMap::new() };

    let mut next_dirs = VecDeque::new();

    next_dirs.push_back(root_path.clone());

    while let Some(current_dir_rel_path) = next_dirs.pop_front() {
        let current_dir_abs_path = root_path.join(&current_dir_rel_path);
        let readdir = match fs::read_dir(current_dir_abs_path) {
            Ok(dir) => dir,
            Err(err) => {
                println!("Could not open dir {:?}: {:?}", &current_dir_rel_path, err);
                continue;
            }
        };

        for entry in readdir {
            let entry = entry.expect("could not read dir");

            let name = entry.file_name();
            let rel_path = current_dir_rel_path.join(&name);
            let typ = entry.file_type().expect("could not get file type");

            let entry = if typ.is_dir() {
                next_dirs.push_back(rel_path.clone());

                Entry::Dir
            } else if typ.is_file() {
                Entry::File
            } else {
                continue;
            };

            state.entries.insert(rel_path, entry);
        }
    }

    Ok(state)
}