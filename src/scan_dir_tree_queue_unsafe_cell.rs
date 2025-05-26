use std::collections::VecDeque;
use std::{fs, thread};
use std::cell::{UnsafeCell};
use std::ffi::OsString;
use std::path::{PathBuf};
use std::time::Duration;
use tokio::time::Instant;

pub fn run() {
    let now = Instant::now();

    let root = read_dir_recursive("/Users/stormbreaker/dev-work".into());

    let elapsed = now.elapsed().as_secs_f32() * 1000f32;

    println!("Done in {elapsed}ms");

    loop {
        thread::sleep(Duration::from_secs(5));

        println!("Paths: {:?}", root.name)
    }
}

struct Node {
    name: OsString,
    data: NodeData
}

enum NodeData {
    // Using UnsafeCell here to allow aliasing mutable references with immutable ones
    Dir(Box<UnsafeCell<DirNode>>),
    File(Box<UnsafeCell<FileNode>>),
}

struct DirNode {
    entries: Vec<Node>
}
struct FileNode {}

struct NextDir {
    rel_path: PathBuf,
    parent_node_ptr: *mut DirNode
}

fn read_dir_recursive(root_path: PathBuf) -> Node {
    let mut next_dirs = VecDeque::new();
    let mut dir_count = 0;
    let mut file_count = 0;

    let root_ptr = Box::new(UnsafeCell::new(DirNode { entries: Vec::new() }));

    next_dirs.push_back(NextDir {
        rel_path: "".into(),
        parent_node_ptr: root_ptr.get()
    });

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
            let rel_path = next.rel_path.join(&name);

            let node_data = if typ.is_dir() {
                dir_count += 1;

                let dir_node = Box::new(UnsafeCell::new(DirNode {
                    entries: Vec::new()
                }));

                next_dirs.push_back(NextDir {
                    rel_path,
                    parent_node_ptr: dir_node.get()
                });

                NodeData::Dir(dir_node)
            } else if typ.is_file() {
                file_count += 1;

                NodeData::File(Box::new(UnsafeCell::new(FileNode {})))
            } else {
                continue;
            };

            {
                let node_to_push = Node {
                    name,
                    data: node_data
                };

                unsafe { (*next.parent_node_ptr).entries.push(node_to_push) }
            }
        }
    }

    println!("Dirs: {dir_count}");
    println!("Files: {file_count}");

    Node {
        name: "".into(),
        data: NodeData::Dir(root_ptr)
    }
}