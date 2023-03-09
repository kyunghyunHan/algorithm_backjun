use std::io::{stdin, Read};
fn main() {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer).unwrap();
    print!("{}", buffer);
}