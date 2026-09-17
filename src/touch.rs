// Copyright (c) 2026 rida-hdj
use std::{env, fs};

pub fn run() {
    // collect args
    let args: Vec<String> = env::args().collect();
    // touch should take args[2] because env::args collect everything when lutils is running

    // repeat create file to all args
    for i in 2..args.len() {
        create_file(&args[i]);
    }
}

// create file
fn create_file(path: &str) {
    // make sure if the file already exists
    if fs::exists(path).unwrap() {
        println!("file already exist: {}", path);
        return;
    }
    // create the file
    let create_file_result = fs::File::create(path);
    let _create_file = match create_file_result {
        Ok(_file) => {},
        Err(error) => println!("lutils: cannot create file '{}': {}", path, error),
    };
}
