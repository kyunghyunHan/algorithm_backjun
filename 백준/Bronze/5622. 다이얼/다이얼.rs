use std::io;
use std::str;
fn main() {
    let arr = [
        3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 10, 10, 10, 10,
    ];
    let mut sum = 0;
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let s: Vec<&str> = input_a.split_whitespace().collect();
    let mut s_arr = s[0].as_bytes();

    for i in 0..s_arr.len() {
        let index = s_arr[i] - 65;
        sum += arr[index as usize];
    }

    println!("{}", sum);
}
