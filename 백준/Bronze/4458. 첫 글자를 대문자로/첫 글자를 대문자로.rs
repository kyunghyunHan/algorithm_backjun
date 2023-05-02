use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let test: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..test {
        let mut str = lines.next().unwrap();
        str[0..1].make_ascii_uppercase(); // convert first character to uppercase

        println!("{}", str);
    }
}
