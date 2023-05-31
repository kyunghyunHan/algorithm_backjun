use std::io;

fn main() {
    let mut first_num = String::new();
    io::stdin().read_line(&mut first_num).expect("Failed to read input");
    let first_num: i32 = first_num.trim().parse().expect("Invalid input");

    let mut len_result = 0;
    let mut result: Vec<i32> = Vec::new();

    for i in 0..=first_num {
        let mut result_list = vec![first_num, i];
        let mut j = 0;
        loop {
            let last_num = result_list[j] - result_list[j + 1];
            j += 1;
            if last_num < 0 {
                break;
            }
            result_list.push(last_num);
            if len_result < result_list.len() {
                len_result = result_list.len();
                result = result_list.clone();
            }
        }
    }

    println!("{}", len_result);
    let final_result: Vec<String> = result.iter().map(|&num| num.to_string()).collect();
    println!("{}", final_result.join(" "));
}
