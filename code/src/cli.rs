
use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;


pub fn main() -> clap::ArgMatches{
    command!()
        .subcommand(
            Command::new("c16")
                .about("Chapter 16")
                .arg(arg!(-e --example "example of chapter 16").required(false))
                .arg(arg!([e] "example to run").required(true))
        )
        .about("Specify the chapter and example to run")
        .get_matches()
}