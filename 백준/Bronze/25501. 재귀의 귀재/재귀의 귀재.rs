use std::io;

fn recursion(s: &str, l: usize, r: usize) -> (bool, usize) {
    let mut cnt = 0;
    cnt += 1;
    if l >= r {
        return (true, cnt);
    } else if s.chars().nth(l) != s.chars().nth(r) {
        return (false, cnt);
    } else {
        let (is_palin, sub_cnt) = recursion(s, l + 1, r - 1);
        cnt += sub_cnt;
        return (is_palin, cnt);
    }
}

fn is_palindrome(s: &str) -> (bool, usize) {
    return recursion(s, 0, s.len() - 1);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<usize>().unwrap();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();
        let (is_palin, cnt) = is_palindrome(s);
        println!("{} {}", if is_palin { 1 } else { 0 }, cnt);
    }
}