use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let source_path = {
        let mut b = PathBuf::new();
        b.push(&env::var("DEP_MY_LIB_RESOURCE_DIR").unwrap());
        b.push("hello.txt");
        b
    };
    eprintln!("source_path: \"{}\"", source_path.to_string_lossy());
    let destination_path = {
        let mut b = PathBuf::new();
        b.push(&env::var("OUT_DIR").unwrap());
        b.push("hello.txt");
        b
    };
    fs::copy(source_path, destination_path).unwrap();
}
