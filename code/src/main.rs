mod c16;
mod cli;
mod math;
mod threading;
mod c15;

use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;


fn main() {

    let matches = cli::main();

    if let Some(sub_match) = matches.subcommand_matches("c16") {
        let flag =  sub_match.get_one::<String>("e").expect("required");

        if flag == "1" {
            c16::e1_thread_usage::main();
        }
        if flag == "2" {
            c16::e2_join_threads::main();
        }
        
    }

}
