use std::collections::VecDeque;
use std::{fs, thread};
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
    Dir(Rc<RefCell<DirNode>>),
    File(Rc<RefCell<FileNode>>),
}

struct DirNode {
    entries: Vec<Node>
}
struct FileNode {}

struct NextDir {
    rel_path: PathBuf,
    parent_node: Rc<RefCell<DirNode>>
}

fn read_dir_recursive(root_path: PathBuf) -> Node {
    let mut next_dirs = VecDeque::new();
    let mut dir_count = 0;
    let mut file_count = 0;

    let root = Rc::new(RefCell::new(DirNode { entries: Vec::new() }));

    next_dirs.push_back(NextDir {
        rel_path: "".into(),
        parent_node: root.clone()
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

                let dir_node = Rc::new(RefCell::new(DirNode {
                    entries: Vec::new()
                }));

                next_dirs.push_back(NextDir {
                    rel_path,
                    parent_node: dir_node.clone()
                });

                NodeData::Dir(dir_node)
            } else if typ.is_file() {
                file_count += 1;

                // println!("- {rel_path:?}")

                NodeData::File(Rc::new(RefCell::new(FileNode {})))
            } else {
                // println!("- {rel_path:?}??")
                continue;
            };

            {
                let mut parent = next.parent_node.borrow_mut();
                parent.entries.push(Node {
                    name,
                    data: node_data
                });
            }
        }
    }

    println!("Dirs: {dir_count}");
    println!("Files: {file_count}");

    Node {
        name: "".into(),
        data: NodeData::Dir(root)
    }
}