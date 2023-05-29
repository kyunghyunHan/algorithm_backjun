use std::io::{BufRead,BufWriter,BufReader,Write,stdin,stdout};

fn cmp(v: &str, i: usize) -> i32 {
    let sen = ["000000", "001111", "010011", "011100", "100110", "101001", "110101", "111010"];
    let mut ret = 0;
    for (j, c) in v.chars().enumerate() {
        if sen[i].chars().nth(j).unwrap() != c {
            ret += 1;
        }
    }
    ret
}

fn main() {
let mut reader= BufReader::new(stdin().lock());
let mut input= String::new();
reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    input.clear();

    reader.read_line(&mut input).unwrap();

    let s = input.trim().to_string();
    let mut ans = String::new();
    for i in 0..n {
        let t = &s[i as usize * 6..(i as usize + 1) * 6];
        let mut ok = false;
        for j in 0..8 {
            if cmp(t, j) <= 1 {
                ok = true;
                ans.push((j as u8 + b'A') as char);
                break;
            }
        }
        if !ok {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", ans);
}
