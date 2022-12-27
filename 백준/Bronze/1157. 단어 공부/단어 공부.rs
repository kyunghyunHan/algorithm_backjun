use std::io;
use std::str;
fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let s: Vec<&str> = input_a.split_whitespace().collect();

    let mut alphabet = [0; 26];
    let mut max_cnt = 0;
    let mut index = 0;
    let mut count = 0;

    for i in 0..s[0].as_bytes().len() {
        if s[0].as_bytes()[i] < 97 {
            alphabet[s[0].as_bytes()[i] as usize - 65 as usize] += 1;
        } else {
            alphabet[s[0].as_bytes()[i] as usize - 97 as usize] += 1;
        }
    }
    for i in 0..26 {
        if alphabet[i] > max_cnt {
            max_cnt = alphabet[i];
            index = i;
        }
    }

    for i in 0..26 {
        if alphabet[i] == max_cnt {
            count += 1;
            if count >= 2 {
                println!("{}", "?");
                return;
            }
        }
    }
    let test: Vec<u8> = vec![index as u8 + 65];
    println!("{}", str::from_utf8(&test).unwrap());
}
