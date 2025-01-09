use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
    let n = input.next().unwrap().unwrap().parse::<usize>().unwrap();
    
    // 팰린드롬인 문자열 개수 세기
    let mut palindrome_count = 0;
    
    for _ in 0..n {
        let s = input.next().unwrap().unwrap();
        if is_palindrome(&s) {
            palindrome_count += 1;
        }
    }
    
    // nP2 = n * (n-1) 계산
    let result = palindrome_count * (palindrome_count - 1);
    
    writeln!(writer, "{}", result).unwrap();
    writer.flush().unwrap();
}

fn is_palindrome(s: &str) -> bool {
    let len = s.len();
    let chars: Vec<char> = s.chars().collect();
    for i in 0..len/2 {
        if chars[i] != chars[len-1-i] {
            return false;
        }
    }
    true
}