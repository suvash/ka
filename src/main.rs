use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn ctrl_byte(c: char) -> u8 {
    c as u8 & &0b0001_1111
}

fn die(e: std::io::Error) {
    panic!(e);
}

fn main() {
    let _raw_stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(by) => {
                let c = by as char;
                println!("{}", c);
                if c.is_control() {
                    println!("{:?} \r", by);
                } else {
                    println!("{:?} ({})\r", by, c);
                }
                if by == ctrl_byte('q') {
                    break;
                }
            }
            Err(err) => die(err),
        }
    }
}
