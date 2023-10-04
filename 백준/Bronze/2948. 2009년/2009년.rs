fn main() {
    let mon = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let w = ["Wednesday", "Thursday", "Friday", "Saturday", "Sunday", "Monday", "Tuesday"];
    
    let mut m = 0;
    let mut n = 0;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let input_parts: Vec<&str> = input.split_whitespace().collect();
    if input_parts.len() == 2 {
        m = input_parts[0].parse().expect("Failed to parse m");
        n = input_parts[1].parse().expect("Failed to parse n");
    } else {
        panic!("Invalid input");
    }

    for i in 1..n {
        m += mon[i];
    }

    println!("{}", w[m % 7]);
}
