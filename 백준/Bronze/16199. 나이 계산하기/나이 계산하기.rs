use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let y1: i32 = iter.next().unwrap().parse().unwrap();
    let m1: i32 = iter.next().unwrap().parse().unwrap();
    let d1: i32 = iter.next().unwrap().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let y2: i32 = iter.next().unwrap().parse().unwrap();
    let m2: i32 = iter.next().unwrap().parse().unwrap();
    let d2: i32 = iter.next().unwrap().parse().unwrap();
    
    let mut man_old = 0;
    if m1 < m2 {
        man_old = y2 - y1;
    } else if m1 == m2 {
        if d1 <= d2 {
            man_old = y2 - y1;
        } else {
            man_old = y2 - y1 - 1;
        }
    } else {
        man_old = y2 - y1 - 1;
    }
    
    let count_old = y2 - y1 + 1;
    let year_old = y2 - y1;
    
    println!("{}", man_old);
    println!("{}", count_old);
    println!("{}", year_old);
}
