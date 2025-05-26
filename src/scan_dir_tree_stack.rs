use std::{fs, thread};
use std::ffi::{OsString};
use std::fmt::{Debug};
use std::fs::{DirEntry, FileType};
use std::path::{PathBuf};
use std::time::Duration;
use tokio::time::Instant;

pub fn run() {
    let now = Instant::now();

    let root = scan_dir_recursive("/Users/stormbreaker/dev-work".into());

    let elapsed = now.elapsed().as_secs_f32() * 1000f32;

    println!("Done in {elapsed}ms");

    loop {
        thread::sleep(Duration::from_secs(5));

        // println!("Paths: {:?}", root)
    }
}

#[derive(Debug)]
struct Node {
    name: OsString,
    data: NodeData
}

#[derive(Debug)]
enum NodeData {
    Dir(DirNode),
    File(FileNode),
}

#[derive(Debug)]
struct DirNode {
    entries: Vec<Node>
}

#[derive(Debug)]
struct FileNode {}

struct WIPDir {
    name: OsString,
    rest_entries: Vec<WIPDirEntry>,
    results: Vec<Node>
}

struct WIPDirEntry {
    name: OsString,
    abs_path: PathBuf
}

fn scan_dir_recursive(root_path: PathBuf) -> Node {
    let mut todo = Vec::new();

    let root_wip_dir = read_dir(OsString::from(""), root_path)
        .expect("could not read root dir");
    todo.push(root_wip_dir);

    while todo.len() > 0 {
        let (done, to_push) = {
            let last = todo.len() - 1;
            let wip_dir = &mut todo[last];

            match wip_dir.rest_entries.pop() {
                Some(entry) => (false, read_dir(entry.name, entry.abs_path)),
                None => (true, None)
            }
        };

        match to_push {
            None => {}
            Some(wip_dir) => { todo.push(wip_dir) }
        }

        if done {
            let current = todo.pop().unwrap();

            let node = Node {
                name: current.name,
                data: NodeData::Dir(DirNode { entries: current.results })
            };

            match todo.last_mut() {
                None => return node,
                Some(parent) => parent.results.push(node)
            }
        }
    }

    unreachable!()
}

fn read_dir(name: OsString, path: PathBuf) -> Option<WIPDir> {
    let entries = match fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(err) => {
            println!("Could not open dir {:?}: {:?}", path, err);
            return None
        }
    };

    let mut files = Vec::new();
    let mut dirs = Vec::new();

    for entry in entries {
        match entry {
            Ok(e) => {
                let t = e.file_type().unwrap();
                if t.is_file() {
                    files.push(Node {
                        name: e.file_name(),
                        data: NodeData::File(FileNode {})
                    });
                } else if t.is_dir() {
                    dirs.push(WIPDirEntry {
                        name: e.file_name(),
                        abs_path: e.path(),
                    });
                } else {
                    continue
                }
            }
            Err(_) => continue
        }
    }

    Some(WIPDir {
        name,
        rest_entries: dirs,
        results: files,
    })
}