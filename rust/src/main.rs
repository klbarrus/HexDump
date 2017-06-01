extern crate hexdump;

use std::env;
use std::process;
use hexdump::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        show_usage();
        process::exit(1);
    });

//    println!("in file: {}", config.in_file);

    if let Err(e) = hexdump::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn show_usage() {
    println!("Hexdump.");
    println!("");
    println!("Usage:");
    println!("    hexdump infile");
    println!("");
}