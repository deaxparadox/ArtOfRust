use std::error::Error;
use std::process;
use std::env;
use std::fs;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Seraching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    // 
    // or
    // 
    // minigrep::run(config).unwrap_or_else(|err| {
    //     println!("Application error: {err}");
    //     process::exit(0);
    // })
}

