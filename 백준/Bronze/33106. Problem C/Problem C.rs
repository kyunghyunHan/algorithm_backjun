use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // 입력 개수 읽기
    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..n {
        let word = lines.next().unwrap().unwrap();
        let chars: Vec<char> = word.chars().collect();
        let mut result = String::new();
        let mut i = 0;
        
        while i < chars.len() {
            if i + 1 < chars.len() && chars[i] == 'c' && chars[i + 1] == 'h' {
                result.push('c');
                i += 2;
                continue;
            }
            
            if chars[i] == 'c' {
                if i + 1 < chars.len() {
                    let next = chars[i + 1];
                    if next == 'e' || next == 'i' || next == 'y' {
                        result.push('s');
                    } else if next == 'a' || next == 'o' || next == 'u' ||
                              (next != 'h' && next != 'y' && 
                               !['a','e','i','o','u'].contains(&next)) {
                        result.push('k');
                    }
                } else {
                    result.push('k');  // 단어의 끝
                }
            } else {
                result.push(chars[i]);
            }
            i += 1;
        }
        
        println!("{}", result);
    }
}