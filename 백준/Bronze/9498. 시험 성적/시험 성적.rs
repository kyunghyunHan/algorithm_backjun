use std::io;
fn main() {
    //숫자를 입력받아서
    let mut numbersArry = String::new();
    //숫자를 입력받아서
    io::stdin().read_line(&mut numbersArry).unwrap();
    // collect()벡터
    let numbers: Vec<&str> = numbersArry.split_whitespace().collect();
    //string값을 숫자로 변환
    let number_first = numbers[0].parse::<i32>().unwrap();
    match number_first {
        90..=100 => println!("A"),
        80..=89 => println!("B"),
        70..=79 => println!("C"),
        60..=69 => println!("D"),
        _ => println!("F"),
    }
}
