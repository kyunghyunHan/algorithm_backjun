use std::io;

fn main() {
    //숫자를 입력받아서
    let mut numbersArry = String::new();
    //숫자를 입력받아서
    io::stdin().read_line(&mut numbersArry).unwrap();
    // collect()벡터
    let numbers: Vec<&str> = numbersArry.split_whitespace().collect();
    //string값을 숫자로 변환
    let number_first: i32 = numbers[0].parse::<i32>().unwrap();
    let number_second: i32 = numbers[1].parse::<i32>().unwrap();
    println!("{}", number_first + number_second);
    println!("{}", number_first - number_second);
    println!("{}", number_first * number_second);
    println!("{}", number_first / number_second);
    println!("{}", number_first % number_second);
}
