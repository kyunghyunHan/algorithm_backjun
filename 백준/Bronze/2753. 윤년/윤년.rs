//윤년
use std::io;
fn main() {
    //숫자를 입력받아서
    let mut numbersArry = String::new();
    //숫자를 입력받아서
    io::stdin().read_line(&mut numbersArry);
    // collect()벡터
    let numbers: Vec<&str> = numbersArry.split_whitespace().collect();
    //string값을 숫자로 변환
    let number_first = numbers[0].parse::<i32>().unwrap();
    if number_first % 4 == 0 && number_first % 100 != 0 || number_first % 400 == 0 {
        println!("{}", "1")
    } else {
        println!("{}", "0")
    }
}
