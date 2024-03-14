use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    loop {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let a: String = match parts.next() {
            Some(val) => val.to_string(),
            None => break,
        };
        let b: String = match parts.next() {
            Some(val) => val.to_string(),
            None => break,
        };
        if a == "0" && b == "0" {
            break;
        }
        let mut a = a;
        let mut b = b;
        if a.len() > b.len() {
            b = "0".repeat(a.len() - b.len()) + &b;
        } else {
            a = "0".repeat(b.len() - a.len()) + &a;
        }
        let mut cnt = 0;
        let mut carry = 0;
        for i in (0..a.len()).rev() {
            let sum = a.chars().nth(i).unwrap().to_digit(10).unwrap() +
                      b.chars().nth(i).unwrap().to_digit(10).unwrap() +
                      carry;
            if sum >= 10 {
                cnt += 1;
                carry = 1;
            } else {
                carry = 0;
            }
        }
        writeln!(writer,"{}",cnt).unwrap();
    }
    writer.flush().unwrap();
}
