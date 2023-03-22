use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let mut input_iter = input.split_whitespace();
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let m: usize = input_iter.next().unwrap().parse().unwrap();
    let mut list: Vec<usize> = (1..=n).collect();

    for _ in 0..m {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let mut input_iter = input.split_whitespace();
        let i: usize = input_iter.next().unwrap().parse().unwrap();
        let j: usize = input_iter.next().unwrap().parse().unwrap();
        let k: usize = input_iter.next().unwrap().parse().unwrap();

        let mut val = list[i - 1];
        let end = list[k - 1];

        while val != end {
            list.remove(i - 1);
            list.insert(j - 1, val);
            val = list[i - 1];
        }
    }

    let result: Vec<String> = list.iter().map(|&x| x.to_string()).collect();
    println!("{}", result.join(" "));
}