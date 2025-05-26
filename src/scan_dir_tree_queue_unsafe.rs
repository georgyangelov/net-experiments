use std::collections::VecDeque;
use std::{fs, mem, ptr, thread};
use std::cell::RefCell;
use std::ffi::OsString;
use std::path::{PathBuf};
use std::rc::Rc;
use std::time::Duration;
use tokio::time::Instant;

pub fn run() {
    let now = Instant::now();

    let root = read_dir_recursive("/Users/stormbreaker/dev-work".into());

    let elapsed = now.elapsed().as_secs_f32() * 1000f32;

    println!("Done in {elapsed}ms");

    // loop {
    //     thread::sleep(Duration::from_secs(5));
    //
    //     println!("Paths: {:?}", root.name)
    // }
}

struct Node {
    name: OsString,
    data: NodeData
}

enum NodeData {
    Dir(*mut DirNode),
    File(*mut FileNode),
}

impl Drop for NodeData {
    fn drop(&mut self) {
        match self {
            NodeData::Dir(dir_node_ptr) => {
                let ptr = mem::replace(dir_node_ptr, ptr::null_mut());

                drop(unsafe { Box::from_raw(ptr) })
            }
            NodeData::File(file_ptr) => {
                let ptr = mem::replace(file_ptr, ptr::null_mut());

                drop(unsafe { Box::from_raw(ptr) })
            }
        }
    }
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

    let root_ptr = Box::into_raw(Box::new(DirNode { entries: Vec::new() }));

    next_dirs.push_back(NextDir {
        rel_path: "".into(),
        parent_node_ptr: root_ptr
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

            // paths.push(rel_path.clone());

            let node_data = if typ.is_dir() {
                // println!("- {rel_path:?}/");
                dir_count += 1;

                let dir_node = Box::into_raw(Box::new(DirNode {
                    entries: Vec::new()
                }));

                next_dirs.push_back(NextDir {
                    rel_path,
                    parent_node_ptr: dir_node
                });

                NodeData::Dir(dir_node)
            } else if typ.is_file() {
                file_count += 1;

                // println!("- {rel_path:?}")

                NodeData::File(Box::into_raw(Box::new(FileNode {})))
            } else {
                // println!("- {rel_path:?}??")
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