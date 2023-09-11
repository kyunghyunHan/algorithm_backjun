use std::io;
fn main() {
    //숫자를 입력받아서
    let mut numbersArry = String::new();
    //숫자를 입력받아서
    io::stdin().read_line(&mut numbersArry).unwrap();
    // collect()벡터
    let numbers: Vec<&str> = numbersArry.split_whitespace().collect();
    //string값을 숫자로 변환
    let number_first: f64 = numbers[0].parse::<f64>().unwrap();
    let number_second: f64 = numbers[1].parse::<f64>().unwrap();

    if number_first > number_second {
        println!("{}", ">");
    } else if number_first < number_second {
        println!("{}", "<");
    } else {
        println!("{}", "==");
    }
}
