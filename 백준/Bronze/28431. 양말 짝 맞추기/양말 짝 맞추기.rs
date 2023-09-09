use std::collections::HashSet;
use std::io;

fn main() {
    let mut lst = HashSet::new();
    let mut res = 0;

    for _ in 0..5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let a: i32 = input.trim().parse().expect("Failed to parse input");

        if lst.contains(&a) {
            res -= a;
            lst.remove(&a);
        } else {
            lst.insert(a);
            res += a;
        }
    }

    println!("{}", res);
}
