extern crate termion;

use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::TermRead;
use termion::clear;
use termion::cursor;

use std::io::{Write, Read, Stdout, stdout, stdin};

use file_buffer::FileBuffer;

pub struct Editor {
    buffer: FileBuffer,
    stdout: RawTerminal<Stdout>,
    size: (u16, u16),
    pos: (u16, u16),
}

impl Editor {
    pub fn new(buffer: FileBuffer) -> Editor {
        let stdout = stdout().into_raw_mode().unwrap();
        let (width, height) = termion::terminal_size().unwrap();

        Editor {
            buffer: buffer,
            stdout: stdout,
            size: (width, height),
            pos: (1,1),
        }
    }

    fn move_to_top(&mut self) {
        self.pos = (1,1);
        write!(self.stdout, "{}", cursor::Goto(1,1));
        self.stdout.flush().unwrap();
    }

    fn print_buffer_contents(&mut self) {

        // print each line in the buffer
        let contents = self.buffer.get_contents();
        for line in contents.lines() {
            write!(self.stdout, "{}\n\r", line).unwrap();
        }
        self.stdout.flush().unwrap();
    }

    fn move_to(&mut self, m_x: i16, m_y: i16) {
        let (x, y) = self.pos;
        let (w, h) = self.size;

        let d_x = if (x as i16) + m_x < 0 {
            // if we tried to move past the beginning of the line, then
            // we just cap it at the beginning of th eline
            // TODO update x to the newline character for y value
            0
        } else if (x as i16) + m_x > w as i16 {
            // if we tried to move past the end of the line, then we just
            // cap it to the end of the line
            // TODO update to the newline character for this line :)
            w
        } else {
            // TODO cap to the newline character
            (x as i16 + m_x) as u16
        };

        // similar rules for y
        let d_y = if (y as i16) + m_y < 0 as i16 {
            0
        } else if (y as i16) + m_y > h as i16 {
            h
        } else {
            (y as i16 + m_y) as u16
        };

        self.pos = (d_x, d_y);

        // move the termion cursor
        let (x, y) = self.pos;
        write!(self.stdout, "{}", cursor::Goto(x, y)).unwrap();
    }

    fn move_to_start_of_next_line(&mut self) {
        write!(self.stdout, "\n\r").unwrap();
        let (_,y) = self.pos;
        self.pos = (0, y + 1);
    }

    fn listen_to_key_events(&mut self) {
        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    write!(self.stdout, "{}", clear::All).unwrap();
                    break;
                },
                Key::Char('\n') => {
                    self.move_to_start_of_next_line();
                }
                Key::Left | Key::Char('h') => {
                    self.move_to(-1, 0);
                }
                Key::Right | Key::Char('l') => {
                    self.move_to(1, 0);
                }
                Key::Up | Key::Char('j') => {
                    self.move_to(0, -1);
                }
                Key::Down | Key::Char('k') => {
                    self.move_to(0, 1);
                }
                Key::Char(c) => {
                    self.move_to(1, 0);
                    write!(self.stdout, "{}", c).unwrap();
                }
                _ => break,
            }
            self.stdout.flush().unwrap();
        }
    }

    fn clear_screen(&mut self) {
        write!(self.stdout, "{}", clear::All);
    }

    pub fn start(&mut self) {
        self.clear_screen();

        self.move_to_top();
        self.print_buffer_contents();
        self.move_to_top();

        self.listen_to_key_events();

        // self.buffer.watch().unwrap();
    }
}
