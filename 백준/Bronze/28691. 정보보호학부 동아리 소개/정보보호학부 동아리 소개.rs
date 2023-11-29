use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first_char = input.trim();

    match first_char {
        "M" => println!("MatKor"),
        "W" => println!("WiCys"),
        "C" => println!("CyKor"),
        "A" => println!("AlKor"),
        "$" => println!("$clear"),
        _ => println!("Invalid input"),
    }
}
