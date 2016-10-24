#[macro_use]
extern crate clap;

extern crate notify;
extern crate termion;
extern crate yate;

use yate::file_buffer::FileBuffer;
use yate::editor::Editor;

fn main() {
    use clap::App;

    let yml = load_yaml!("args.yaml");
    let m = App::from_yaml(yml).get_matches();

    // first parse arguments
    if let Some(filename) = m.value_of("filename") {
        // read the files
        match FileBuffer::new(String::from(filename)) {
            Ok(buffer) => {
                let mut editor = Editor::new(buffer);
                editor.start();
            }
            Err(e) => {
                println!("Error {}", e);
            }
        }
    }
}
