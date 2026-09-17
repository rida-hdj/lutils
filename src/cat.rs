use std::{fs, env};

pub fn run() {
    // collect args
    let args: Vec<String> = env::args().collect();
    // cat should take args[2] because env::args collect everything when lutils is running
    
    // repeat print string to all args
    for i in 2..args.len() {
        print_string(&args[i]);
    }
}

// print file content
fn print_string(path: &str) {
    // print content
    let read_file_resault = fs::read_to_string(path);
    let _read_file = match read_file_resault {
        Ok(content) => print!("{}", content),
        Err(error) => println!("lutils: cannot create file '{}': {}", path, error),
    };
}
