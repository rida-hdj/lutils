use std::{env, fs};

pub fn run() {
    let args: Vec<String> = env::args().collect();
    // mkdir should take args[2] because env::args collect everything when lutils running
    for i in 2..args.len() {
        create_dir(&args[i]);
    }
}

fn create_dir(path: &str) {
    let create_dir_result = fs::create_dir(path);
    let _create_dir = match create_dir_result {
        Ok(()) => {},
        Err(error) => println!("lutils: cannot create directory '{}': {}", path, error),
    };
}
