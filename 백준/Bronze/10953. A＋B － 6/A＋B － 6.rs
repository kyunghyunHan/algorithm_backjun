use std::io;

fn a_plus_b_6(number_list: &[(i32, i32)]) -> Vec<i32> {
    number_list.iter().map(|(num1, num2)| num1 + num2).collect()
}

fn main() {
    let mut number_list = Vec::new();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input");

    for _ in 0..n {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let nums: Vec<i32> = input
            .split(',')
            .map(|num| num.trim().parse().expect("Invalid input"))
            .collect();

        number_list.push((nums[0], nums[1]));
    }

    let result = a_plus_b_6(&number_list);

    for num in result {
        println!("{}", num);
    }
}
