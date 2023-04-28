use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Remove any leading or trailing whitespace from the input
        let input = input.trim();

        if input == "0" {
            break;
        }

        let is_palindrome = input.chars().zip(input.chars().rev()).all(|(a, b)| a == b);
        if is_palindrome {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
