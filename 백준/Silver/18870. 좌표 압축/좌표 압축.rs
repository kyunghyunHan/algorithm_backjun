use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let vec: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut copy_vec = vec.clone();
    copy_vec.sort_unstable();
    copy_vec.dedup();

    for i in vec {
        let idx = match copy_vec.binary_search(&i) {
            Ok(x) => x,
            Err(x) => x,
        };
        print!("{} ", idx);
    }
}