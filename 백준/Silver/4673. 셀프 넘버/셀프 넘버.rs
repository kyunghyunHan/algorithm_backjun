use std::io;
//input_a

fn d(mut num: usize) -> usize {
    let mut sum = num.clone();

    while num != 0 {
        sum = sum + (num % 10);
        num = num / 10;
    }
    sum
}
fn main() {
    // let check: [usize; 10001];
    let mut check: Vec<bool> = vec![false; 10001];
    for i in 1..10001 {
        let n = d(i);
        if n < 10001 {
            check[n] = (true);
        }
    }

    for i in 1..10001 {
        if !check[i] {
            println!("{}", i)
        }
    }
}
