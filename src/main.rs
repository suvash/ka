use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    let _raw_stdout = stdout().into_raw_mode().unwrap();

    for key in io::stdin().keys() {
        match key {
            Ok(k) => match k {
                Key::Char(c) => {
                    if c.is_control() {
                        println!("{:?} \r", c as u8);
                    } else {
                        println!("{:?} ({})\r", c as u8, c);
                    }
                }
                Key::Ctrl('q') => {
                    println!("Exiting");
                    break;
                }
                _ => println!("{:?}\r", k),
            },
            Err(err) => die(err),
        }
    }
}
