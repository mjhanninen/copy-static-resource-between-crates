use std::env;

fn main() {
    println!(
        "cargo:resource-dir={}/resources",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}
