use std::io::{self, Read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    
}

impl Editor {
    pub fn default() -> Self {
        Editor{}
    }
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        for b in io::stdin().bytes() {
            match b {
                Ok(b) => {
                    let c = b as char;
                    if c.is_control() {
                        println!("Binary: {b:08b} ASCII: {b:#03} \r");
                    } else {
                        println!("Binary: {b:08b} ASCII: {b:#03} Character: {c:#?} \r");
                    }
                    if c == 'q' {
                        break;
                    }
                }
                Err(err) => println!("Error: {err}"),
            }
        }
        disable_raw_mode().unwrap();
    }
}