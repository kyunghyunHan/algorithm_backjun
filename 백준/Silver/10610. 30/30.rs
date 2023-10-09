use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input");

    let n = n.trim();

    if !n.contains('0') {
        println!("-1");
    } else {
        let mut num_sum = 0;
        for c in n.chars() {
            num_sum += c.to_digit(10).unwrap_or(0) as i32;
        }

        if num_sum % 3 != 0 {
            println!("-1");
        } else {
            let mut sorted_num: Vec<char> = n.chars().collect();
            sorted_num.sort_by(|a, b| b.cmp(a));
            let answer: String = sorted_num.into_iter().collect();
            println!("{}", answer);
        }
    }
}
