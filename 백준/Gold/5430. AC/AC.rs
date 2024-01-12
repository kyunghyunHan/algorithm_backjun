use std::collections::VecDeque;
use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};


fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut order = String::new();
        reader.read_line(&mut order).unwrap();
        let order = order.trim();

        let mut n_str = String::new();
        reader.read_line(&mut n_str).unwrap();
        let n: usize = n_str.trim().parse().unwrap();

        let mut str = String::new();
        reader.read_line(&mut str).unwrap();
        let str = str.trim();

        let mut dq: VecDeque<i32> = VecDeque::new();
        let mut s = String::new();

        for c in str.chars() {
            if c.is_digit(10) {
                s.push(c);
            } else {
                if !s.is_empty() {
                    dq.push_back(s.parse().unwrap());
                    s.clear();
                }
            }
        }

        let mut reverse = false;
        let mut error = false;

        for o in order.chars() {
            if o == 'R' {
                reverse = !reverse;
            } else {
                if dq.is_empty() {
                    println!("error");
                    error = true;
                    break;
                }
                if reverse {
                    dq.pop_back();
                } else {
                    dq.pop_front();
                }
            }
        }

        if !error {
            print!("[");
        }

        if reverse && !dq.is_empty() {
            for (i, &val) in dq.iter().rev().enumerate() {
                if i == dq.len() - 1 {
                    print!("{}", val);
                } else {
                    print!("{},", val);
                }
            }
        } else if !reverse && !dq.is_empty() {
            for (i, &val) in dq.iter().enumerate() {
                if i == dq.len() - 1 {
                    print!("{}", val);
                } else {
                    print!("{},", val);
                }
            }
        }

        if !error {
            println!("]");
        }
    
    }
}
