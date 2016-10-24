// file reading
use std::io;
use std::io::Read;
use std::fs::File;

// file notifying
use notify;
use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;

// eventually use ropes :)
// use rope::RopeNode;

// the main file buffer
#[derive(Debug)]
pub struct FileBuffer {
    filename: String,
    handle: File,
    contents: String,
}

impl FileBuffer {
    pub fn new(filename: String) -> Result<FileBuffer, io::Error> {
        let mut handle = try!(File::open(&filename));
        let mut contents = String::new();
        try!(handle.read_to_string(&mut contents));

        Ok(FileBuffer {
            filename: filename,
            handle: handle,
            contents: contents,
        })
    }

    pub fn watch(&mut self) -> notify::Result<()> {
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));

        try!(watcher.watch(&self.filename));

        loop {
            match rx.recv() {
                Ok(notify::Event { path: Some(path), op: Ok(op) }) => {
                    println!("{:?} {:?}", op, path);
                }
                Err(e) => println!("watch error {}", e),
                _ => (),
            }
        }
    }

    pub fn get_contents(&self) -> String {
        self.contents.clone()
    }
}
