use std::io;

fn main() {
    let mut first_lst: Vec<i32> = Vec::new();
    let mut second_lst: Vec<(i32, String)> = Vec::new();

    for _ in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let mut iter = input.split_whitespace();
        let P: i32 = iter.next().unwrap().parse().expect("Invalid input");
        let Y: i32 = iter.next().unwrap().parse().expect("Invalid input");
        let S: String = iter.next().unwrap().parse().expect("Invalid input");

        first_lst.push(Y);
        second_lst.push((P, S));
    }

    first_lst.sort();
    second_lst.sort_by(|a, b| b.0.cmp(&a.0));

    let mut first_teamName = String::new();
    let mut second_teamName = String::new();

    for i in 0..3 {
        first_teamName += &(first_lst[i] % 100).to_string();
        second_teamName += &second_lst[i].1[..1];
    }

    println!("{}", first_teamName);
    println!("{}", second_teamName);
}
