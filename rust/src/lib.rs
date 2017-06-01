use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub in_file: String     // name of file to read
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let in_file = args[1].clone();

        Ok(Config {
            in_file: in_file,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut fin = File::open(config.in_file)?;
    let mut buffer: Vec<u8> = Vec::new();
    fin.read_to_end(&mut buffer)?;

    let mut counter = 0;
    let mut offset = 0;
    let mut text: String = String::new();

    for byte in &mut buffer {

        if counter % 16 == 0 {
            print!("{:08x}  ", offset);
            offset += 16;
        }

        print!("{:02x} ", byte);

        let ch = *byte as char;
        if ch.is_control() {
            text = text + ". ";
        } else {
            text.push(ch);
            text.push(' ');
        }

        counter = counter + 1;

        if counter % 16 == 0 {
            counter = 0;
            println!(" -- {}", text);
            text.clear();
        }
    }

    // handle last line if it was less than 16 chars
    if counter != 0 {
        for _ in counter..16 {
            print!("   ");
        }
        println!(" -- {}", text);
        text.clear();
    }

    Ok(())
}