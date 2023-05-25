use std::io;

fn calculate_total_cost(a: i32, b: i32, c: i32, d: i32, e: i32) -> String {
    let price_list: [f64; 5] = [350.34, 230.90, 190.55, 125.30, 180.90];
    
    let parts_num_list: [i32; 5] = [a, b, c, d, e];

    let whole_price: f64 = price_list
        .iter()
        .zip(parts_num_list.iter())
        .map(|(&price, &parts)| price * parts as f64)
        .sum();

    format!("${:.2}", whole_price)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_cases: i32 = input.trim().parse().expect("Invalid input");

    for _ in 0..num_cases {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid input"))
            .collect();

        let a = parts[0];
        let b = parts[1];
        let c = parts[2];
        let d = parts[3];
        let e = parts[4];

        println!("{}", calculate_total_cost(a, b, c, d, e));
    }
}
