use std::io::{stdin, BufRead, BufReader};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap(); // 수정된 부분
    let mut nums = buffer.trim().split_whitespace();
    let n = nums.next().unwrap().parse::<usize>().unwrap();
    let mut s: Vec<i32> = vec![];
    for _ in 0..n {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap(); // 수정된 부분
        let mut nums = buffer.trim().split_whitespace();
        let n = nums.next().unwrap();

        // 1. push
        if n == "push" {
            let n2 = nums.next().unwrap().parse::<i32>().unwrap();
            s.push(n2);
        }
        // 2. pop
        else if n == "pop" {
            if let Some(x) = s.pop() {
                println!("{}", x);
            } else {
                println!("{}", -1);
            }
        }
        // 3. size
        else if n == "size" {
            println!("{}", s.len());
        }
        // 4. empty
        else if n == "empty" {
            if s.is_empty() {
                println!("{}", 1);
            } else {
                println!("{}", 0);
            }
        }
        // 5. top
        else if n == "top" {
            if let Some(&x) = s.last() {
                println!("{}", x);
            } else {
                println!("{}", -1);
            }
        }
    }
}