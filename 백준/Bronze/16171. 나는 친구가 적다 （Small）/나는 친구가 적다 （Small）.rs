use std::io;

fn main() {
   
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_string();

   
    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword).expect("Failed to read keyword");
    let keyword = keyword.trim().to_string();

    let filtered_input: String = input.chars().filter(|c| c.is_alphabetic()).collect();

    if filtered_input.contains(&keyword) {
        println!("1");
    } else {
        println!("0");
    }
}
