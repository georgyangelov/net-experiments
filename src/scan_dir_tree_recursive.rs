use std::collections::VecDeque;
use std::{fs, thread};
use std::cell::RefCell;
use std::ffi::{OsStr, OsString};
use std::path::{PathBuf};
use std::rc::Rc;
use std::time::Duration;
use tokio::time::Instant;

pub fn run() {
    let now = Instant::now();

    let name = OsString::from("");
    let root = read_dir_recursive(&name, "/Users/stormbreaker/dev-work".into()).unwrap();

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
    Dir(DirNode),
    File(FileNode),
}

struct DirNode {
    entries: Vec<Node>
}
struct FileNode {}

fn read_dir_recursive(name: &OsStr, path: PathBuf) -> Option<Node> {
    let mut dir_node = DirNode { entries: Vec::new() };

    let readdir = match fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(err) => {
            println!("Could not open dir {:?}: {:?}", path, err);
            return None;
        }
    };

    for entry in readdir {
        let entry = entry.expect("could not read dir");

        let name = entry.file_name();
        let typ = entry.file_type().expect("could not get file type");
        let rel_path = path.join(&name);

        if typ.is_dir() {
            let inner_node = match read_dir_recursive(&name, rel_path) {
                None => continue,
                Some(nodes) => nodes
            };

            dir_node.entries.push(inner_node);
        } else if typ.is_file() {
            dir_node.entries.push(Node {
                name,
                data: NodeData::File(FileNode {})
            });
            // NodeData::File(Rc::new(RefCell::new(FileNode {})))
        } else {
            continue;
        };
    }

    Some(Node {
        name: name.to_os_string(),
        data: NodeData::Dir(dir_node)
    })
}