use std::collections::{HashMap, VecDeque};
use std::{fs, thread};
use std::ffi::OsString;
use std::path::{PathBuf};
use std::time::Duration;
use tokio::time::Instant;

pub fn run() {
    let now = Instant::now();

    let root = scan_dir_recursively("/Users/stormbreaker/dev-work".into());

    let elapsed = now.elapsed().as_secs_f32() * 1000f32;

    println!("Done in {elapsed}ms");

    loop {
        thread::sleep(Duration::from_secs(5));

        println!("Total objects: {:?}", root.entries.len())
    }
}

struct State {
    root_id: u64,
    entries: HashMap<u64, Entry>
}

struct Entry {
    name: OsString,
    value: EntryValue
}

enum EntryValue {
    Dir(Vec<u64>),
    File
}

struct WIPDir {
    id: u64,
    abs_path: PathBuf
}

fn scan_dir_recursively(root_path: PathBuf) -> State {
    let mut next_i = 1;
    let mut entries: HashMap<u64, Entry> = HashMap::new();

    let mut next_dirs = VecDeque::new();

    next_dirs.push_back(WIPDir {
        id: 0,
        abs_path: root_path.clone()
    });

    entries.insert(0, Entry {
        name: "".into(),
        value: EntryValue::Dir(Vec::new())
    });

    while let Some(wip_dir) = next_dirs.pop_front() {
        let current_dir_abs_path = root_path.join(wip_dir.abs_path);
        let readdir = match fs::read_dir(&current_dir_abs_path) {
            Ok(dir) => dir,
            Err(err) => {
                println!("Could not open dir {:?}: {:?}", &current_dir_abs_path, err);
                continue;
            }
        };

        for entry in readdir {
            let entry = entry.expect("could not read dir");
            let name = entry.file_name();
            let typ = entry.file_type().expect("could not get file type");

            let id = next_i;
            next_i += 1;

            let value = if typ.is_dir() {
                next_dirs.push_back(WIPDir {
                    id,
                    abs_path: entry.path()
                });

                EntryValue::Dir(Vec::new())
            } else if typ.is_file() {
                EntryValue::File
            } else {
                continue;
            };

            entries.insert(id, Entry { name, value });

            let parent_entry = entries.get_mut(&wip_dir.id).unwrap();
            let parent_dir_entries = match &mut parent_entry.value {
                EntryValue::Dir(entries) => entries,
                EntryValue::File => panic!()
            };
            parent_dir_entries.push(id);
        }
    }

    State {
        root_id: 0,
        entries
    }
}