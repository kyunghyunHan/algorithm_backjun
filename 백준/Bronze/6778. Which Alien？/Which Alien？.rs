use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let antenna: i32 = iter.next().unwrap().parse().unwrap();


    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let mut iter2 = input2.trim().split_whitespace();
    let eyes: i32 = iter2.next().unwrap().parse().unwrap();

    if antenna >= 3 && eyes <= 4 {
        println!("TroyMartian");
    }
    if antenna <= 6 && eyes >= 2 {
        println!("VladSaturnian");
    }
    if antenna <= 2 && eyes <= 3 {
        println!("GraemeMercurian");
    }
}
