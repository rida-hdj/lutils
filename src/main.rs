// Copyright (c) 2026 rida-hdj
use std::env::{self};
mod pwd;
mod mkdir;
mod touch;
mod cat;

fn main() {
    // collect args
    let args: Vec<String> = env::args().collect();

    // return if there are no subcommands
    if args.len() < 2 {
        return;
    }

    // match subcommands to the right function
    match args[1].as_str() {
        "help" => help(),
        "pwd" => pwd::run(),
        "mkdir" => mkdir::run(),
        "touch" => touch::run(),
        "cat" => cat::run(),
        _ => println!("there is no command {}, try help", args[1]),
    }
}

// just a help message
fn help() {
    println!("
lutils

usage:
    lutils <command> [argument]

commands:
    pwd             print the current directory
    mkdir <dir>     create a directory
    touch <file>    create a file

help:
    lutils help
");
}
