// Copyright (c) 2026 rida-hdj
use std::{env, fs, io::Write};

pub fn run() {
    // collect args
    let args: Vec<String> = env::args().collect();
    // cat should take args[2] because env::args collect everything when lutils is running

    // use current dir as a default path
    let path = if args.len() > 2 { &args[2] } else { "." };

    // run the list_dir function
    list_dir(path);
}

// print dir content
fn list_dir(path: &str) {
    // try open the dir
    let read_dir_result = fs::read_dir(path);

    // process each entry in the dir
    let _read_dir = match read_dir_result {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    // print entry name
                    Ok(entry) => {
                        // skip hidden files and dirs
                        let name = entry.file_name();
                        let name = name.to_string_lossy();
                        if name.starts_with(".") {
                            continue;
                        }
                        print!("{}  ", name);
                        let _ = std::io::stdout().flush().unwrap();
                    }
                    // handle errors
                    Err(error) => {
                        println!("cannot read entry: {}", error)
                    }
                }
            }
        }
        Err(error) => println!("lutils: cannot read dir '{}': {}", path, error),
    };
    println!("");
}
