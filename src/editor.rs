extern crate termion;

use termion::event::*;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::{TermRead, MouseTerminal};
use termion::clear;
use termion::cursor;

use std::io::{Write, Stdout, stdout, stdin};

use file_buffer::FileBuffer;

pub struct Editor {
    buffer: FileBuffer,
    stdout: MouseTerminal<RawTerminal<Stdout>>,
    size: (u16, u16),
    pos: (u16, u16),
}

enum EditorCommand {
    Quit,
    MoveCursor(i16, i16),
    SetPosition(u16, u16),
    MoveCursorStartOfNextLine,
    TypeChar(char),
    DoNothing,
}

impl Editor {
    pub fn new(buffer: FileBuffer) -> Editor {
        let stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
        let (width, height) = termion::terminal_size().unwrap();

        Editor {
            buffer: buffer,
            stdout: stdout,
            size: (width, height),
            pos: (1, 1),
        }
    }

    fn move_to_top(&mut self) {
        self.set_position(1, 1);
    }

    fn print_buffer_contents(&mut self) {
        // print each line in the buffer
        let contents = self.buffer.get_contents();
        for line in contents.lines() {
            write!(self.stdout, "{}\n\r", line).unwrap();
            self.stdout.flush().unwrap();
        }
    }

    fn set_position(&mut self, x: u16, y: u16) {
        self.pos = (x, y);
        write!(self.stdout, "{}", cursor::Goto(x, y)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn move_cursor(&mut self, m_x: i16, m_y: i16) {
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

        // finally, set the position
        self.set_position(d_x, d_y);
    }

    fn move_to_start_of_next_line(&mut self) {
        write!(self.stdout, "\n\r").unwrap();
        let (_, y) = self.pos;
        self.set_position(0, y + 1);
    }

    fn handle_key_event(&mut self, key: Key) -> EditorCommand {
        match key {
            Key::Char('q') => EditorCommand::Quit,
            Key::Char('\n') => EditorCommand::MoveCursorStartOfNextLine,
            Key::Left | Key::Char('h') => EditorCommand::MoveCursor(-1, 0),
            Key::Right | Key::Char('l') => EditorCommand::MoveCursor(1, 0),
            Key::Up | Key::Char('j') => EditorCommand::MoveCursor(0, -1),
            Key::Down | Key::Char('k') => EditorCommand::MoveCursor(0, 1),
            Key::Char(c) => EditorCommand::TypeChar(c),
            _ => EditorCommand::DoNothing,
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> EditorCommand {
        match mouse_event {
            MouseEvent::Press(_, a, b) |
            MouseEvent::Release(a, b) |
            MouseEvent::Hold(a, b) => EditorCommand::SetPosition(a, b),
        }
    }

    fn handle_events(&mut self) {
        let stdin = stdin();

        for c in stdin.events() {
            let evt = c.unwrap();
            let command = match evt {
                Event::Key(k_event) => self.handle_key_event(k_event),
                Event::Mouse(m_event) => self.handle_mouse_event(m_event),
                _ => EditorCommand::DoNothing,
            };

            let cmd = self.process_command(command);
            match cmd {
                EditorCommand::Quit => break,
                _ => {}
            }
        }
    }

    fn process_command(&mut self, command: EditorCommand) -> EditorCommand {
        match command {
            EditorCommand::Quit => {
                write!(self.stdout, "{}\n\r", clear::All).unwrap();
                EditorCommand::Quit
            }
            EditorCommand::MoveCursor(x, y) => {
                self.move_cursor(x, y);
                EditorCommand::DoNothing
            }
            EditorCommand::SetPosition(x, y) => {
                self.set_position(x, y);
                EditorCommand::DoNothing
            }
            EditorCommand::MoveCursorStartOfNextLine => {
                self.move_to_start_of_next_line();
                EditorCommand::DoNothing
            }
            EditorCommand::TypeChar(c) => {
                self.move_cursor(1, 0);
                write!(self.stdout, "{}", c).unwrap();
                EditorCommand::DoNothing
            }
            EditorCommand::DoNothing => EditorCommand::DoNothing,
        }
    }

    fn clear_screen(&mut self) {
        write!(self.stdout, "{}", clear::All).unwrap();
    }

    pub fn start(&mut self) {
        self.clear_screen();

        // print the buffer
        self.move_to_top();
        self.print_buffer_contents();
        self.move_to_top();

        self.handle_events();

        // self.buffer.watch().unwrap();
    }
}
