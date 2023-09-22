use std::io::{self, Read};
fn main() {
    let input = io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as i32);
    println!("{}", input.unwrap());
}
