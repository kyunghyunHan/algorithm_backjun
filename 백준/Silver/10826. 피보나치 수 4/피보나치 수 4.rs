use std::{cmp, io};
use std::io ::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn add_fibonacci(a: &String, b: &String) -> String {
    let mut result = String::from("0");
    result = result.repeat(cmp::max(a.len(), b.len()));

    let mut carry = false;

    for i in 0..result.len() {
        let mut temp = carry as i64;
        carry = false;

        if i < a.len() {
            temp += (a.as_bytes()[a.len() - i - 1] - 48) as i64;
        }
        if i < b.len() {
            temp += (b.as_bytes()[b.len() - i - 1] - 48) as i64;
        }

        if temp >= 10 {
            carry = true;
            temp -= 10;
        }

        result.replace_range(
            (result.len() - i - 1)..(result.len() - i),
            String::from_utf8(vec![(temp + 48) as u8]).unwrap().as_str(),
        );
    }

    if carry {
        result.insert(0, '1');
    }

    result
}

fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    
    let n:usize = input.trim().parse().unwrap();

    if n == 0 {
        writeln!(writer,"0").unwrap();
        return;
    } else if n == 1 || n == 2 {
        writeln!(writer,"1").unwrap();
        return;
    }

    let mut a = "0".to_string();
    let mut b = "1".to_string();
    let mut ans = String::new();

    for _ in 0..=(n - 2) {
        ans = add_fibonacci(&a, &b);
        a = b.clone();
        b = ans.clone();
    }

    writeln!(writer,"{ans}").unwrap();
}