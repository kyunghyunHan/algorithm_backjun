use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let lambda: u32 = input.trim().parse().unwrap();

    match lambda {
        380..=424 => println!("Violet"),
        425..=449 => println!("Indigo"),
        450..=494 => println!("Blue"),
        495..=569 => println!("Green"),
        570..=589 => println!("Yellow"),
        590..=619 => println!("Orange"),
        620..=780 => println!("Red"),
        _ => println!("Invalid wavelength"),
    }
}