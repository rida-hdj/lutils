use std::{env, fs};

pub fn run() {
    let args: Vec<String> = env::args().collect();
    // touch should take args[2] because env::args collect everything when lutils running
    for i in 2..args.len() {
        create_file(&args[i]);
    }
}

fn create_file(path: &str) {
    if fs::exists(path).unwrap() {
        println!("file already exist: {}", path);
        return;
    }
    let create_file_result = fs::File::create(path);
    let _create_file = match create_file_result {
        Ok(_file) => {},
        Err(error) => println!("lutils: cannot create file '{}': {}", path, error),
    };
}
