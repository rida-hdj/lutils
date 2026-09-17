// Copyright (c) 2026 rida-hdj
use std::{env, fs};

pub fn run() {
    // collect args
    let args: Vec<String> = env::args().collect();
    // rm should take args[2] because env::args collect everything when lutils is running

    // repeat remove file to all args
    for i in 2..args.len() {
        remove_file(&args[i]);
    }
}

// remove file
fn remove_file(path: &str) {
    // check if the file does not exist
    if !fs::exists(path).unwrap() {
        println!("file does not exist: {}", path);
        return;
    }
    // remove the file
    let remove_file_result = fs::remove_file(path);
    let _remove_file = match remove_file_result {
        Ok(()) => {},
        Err(error) => println!("lutils: cannot remove file '{}': {}", path, error),
    };
}
