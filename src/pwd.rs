// Copyright (c) 2026 rida-hdj
// print the current directory
pub fn run() {
    let current_dir = std::env::current_dir().unwrap();
    println!("{}", current_dir.display());
}
