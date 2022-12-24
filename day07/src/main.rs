
mod command;
mod filesystem;

use std::collections::HashMap;
use regex::Regex;

use command::CommandExecution;


fn main() {
    println!("{:#?}", command::read_history("input.txt"));
}
