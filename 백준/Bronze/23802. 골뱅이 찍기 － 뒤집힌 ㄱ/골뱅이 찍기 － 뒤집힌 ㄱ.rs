use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main() {
let mut reader= BufReader::new(stdin().lock());
let mut input = String::new();
reader.read_line(&mut input).unwrap();
let n= input.trim().parse::<usize>().unwrap();

    for _ in 0..n {
        for _ in 0..n * 5 {
            print!("@");
        }
        println!();
    }
    for _ in 0..n * 4 {
        for _ in 0..n {
            print!("@");
        }
        println!();
    }
}
