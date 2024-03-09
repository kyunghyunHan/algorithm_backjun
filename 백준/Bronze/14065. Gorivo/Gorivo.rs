use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let miles_per_gallon: f64 = input.trim().parse()
        .expect("Please enter a valid number for fuel consumption");

    // Convert miles per gallon to liters per 100 kilometers
    let liters_per_100_km = 100.0 / (miles_per_gallon * 1.609344 / 3.785411784);

    println!("{:.6}", liters_per_100_km);
}