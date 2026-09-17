// Copyright (c) 2026 rida-hdj
use std::{env, fs};

pub fn run() {
    // collect args
    let args: Vec<String> = env::args().collect();
    // mkdir should take args[2] because env::args collect everything when lutils is running
    
    // repeat remove directory to all args
    for i in 2..args.len() {
        remove_dir(&args[i]);
    }
}

// remove directory
fn remove_dir(path: &str) {
    // check if the dir does not exist
    if !fs::exists(path).unwrap() {
        println!("directory does not exist: {}", path);
        return;
    }

    let remove_dir_result = fs::remove_dir(path);
    let _remove_dir = match remove_dir_result {
        Ok(()) => {},
        Err(error) => println!("lutils: cannot remove directory '{}': {}", path, error),
    };
}
