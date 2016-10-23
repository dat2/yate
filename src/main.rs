extern crate termion;
extern crate notify;

#[macro_use]
extern crate clap;

extern crate yate;

// file reading
use std::io;
use std::io::Read;
use std::fs::File;

// file notifying
use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;

// termion
use termion::clear;

// built in ropes
use yate::rope::RopeNode;

// the main file buffer
#[derive(Debug)]
struct FileBuffer {
    filename: String,
    handle: File,
    contents: String,
}

impl FileBuffer {
    fn new(filename: String) -> Result<FileBuffer, io::Error> {
        let mut handle = try!(File::open(&filename));
        let mut contents = String::new();
        try!(handle.read_to_string(&mut contents));

        Ok(FileBuffer {
            filename: filename,
            handle: handle,
            contents: contents,
        })
    }

    fn watch(&mut self) -> notify::Result<()> {
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
}

fn main() {
    use clap::App;

    let yml = load_yaml!("args.yaml");
    let m = App::from_yaml(yml).get_matches();

    // first parse arguments
    if let Some(filename) = m.value_of("filename") {
        // read the files
        match FileBuffer::new(String::from(filename)) {
            Ok(mut buffer) => {

                // watch for changes, and prompt an update of contents
                buffer.watch().unwrap();
            }
            Err(e) => {
                println!("Error {}", e);
            }
        }
    }
}
