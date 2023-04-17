use std::io::{stdin, BufRead, BufReader, BufWriter, Write};


fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut buffer = String::new();
    
    reader.read_line(&mut buffer).unwrap();
    let mut nums = buffer.trim().split_whitespace();
    let n = nums.next().unwrap().parse::<usize>().unwrap();
    let m = nums.next().unwrap().parse::<usize>().unwrap();
    if 100*n>=m{
        println!("Yes")
    }else{
        println!("No")
    }
}