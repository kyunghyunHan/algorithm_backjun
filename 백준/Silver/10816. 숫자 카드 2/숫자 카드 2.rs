use std::collections::HashMap;
use std::io::{stdin, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut buffer2 = String::new();
    reader.read_line(&mut buffer2).unwrap();
    let first_arr: Vec<i32> = buffer2
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut buffer3 = String::new();
    reader.read_line(&mut buffer3).unwrap();
    let n2 = buffer3.trim().parse::<usize>().unwrap();

    let mut buffer4 = String::new();
    reader.read_line(&mut buffer4).unwrap();
    let second_arr: Vec<i32> = buffer4
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut counts: HashMap<i32, usize> = HashMap::new();
    for i in first_arr {
        *counts.entry(i).or_insert(0) += 1;
    }

    let result_arr: Vec<usize> = second_arr.iter().map(|i| counts.get(i).unwrap_or(&0).clone()).collect();

    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for i in 0..result_arr.len(){
         write!(writer, "{} ", result_arr[i]).unwrap();
    }
   
}