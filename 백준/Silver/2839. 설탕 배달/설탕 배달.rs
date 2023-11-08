use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<&str> = input.split_whitespace().collect();
    let mut input_value: i32 = numbers[0].parse::<i32>().unwrap();
    let mut Limit = 0;

    while input_value >= 0 {
        if input_value % 5 == 0 {
            Limit += (input_value / 5);
            println!("{}", Limit);
            return;
        } else {
            Limit += 1;
            input_value -= 3;
        }
    }
    println!("{}", -1);
}