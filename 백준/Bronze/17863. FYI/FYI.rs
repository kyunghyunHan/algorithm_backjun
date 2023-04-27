use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");

    if n.starts_with("555") {
        println!("YES");
    } else {
        println!("NO");
    }
}
