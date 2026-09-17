use std::env::{self};
mod pwd;
mod mkdir;
mod touch;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    match args[1].as_str() {
        "help" => help(),
        "pwd" => pwd::run(),
        "mkdir" => mkdir::run(),
        "touch" => touch::run(),
        _ => println!("there is no command {}, try help", args[1]),
    }
}

fn help() {
    println!("
lutils

usage:
    lutils <command> [argument]

commands:
    pwd             print the current directory
    mkdir <dir>     create a directory

help:
    lutils help
");
}
