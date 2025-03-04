mod c16;
mod cli;
mod math;
mod threading;
mod c15;

use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;


fn main() {

    c16::ex15::first();

}
