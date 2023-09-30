//최댓값
use std::io;
fn main() {
    let mut v: Vec<i32> = Vec::new();
    for i in 0..9 {
        let mut numbers_arr2 = String::new();
        io::stdin().read_line(&mut numbers_arr2).unwrap();
        let numbers2: Vec<&str> = numbers_arr2.split_whitespace().collect();
        let number_first2: i32 = numbers2[0].parse::<i32>().unwrap();

        v.push(number_first2);
    }
    let result1 = v.iter().max();

    // let a3 = v.iter().position(|&x| x == 2);
    fn MMMM(item: Option<&i32>) -> i32 {
        match item {
            None => 0,
            Some(i) => *i,
        }
    }
    println!("{}", MMMM(result1));
    for i in 0..v.len() {
        if MMMM(result1) == v[i] {
            println!("{}", i + 1)
        }
    }
}
