fn main() {
    let msg = include_str!(concat!(env!("OUT_DIR"), "/hello.txt"));
    println!("{}", msg);
}
