//1차원배얄
use std::io;
fn main() {
    //스트링
    let mut input = String::new();
    // let mut output = String::new();
    let mut numbers_arry = String::new();
    //입력값받고
    io::stdin().read_line(&mut input).unwrap();
    io::stdin().read_line(&mut numbers_arry).unwrap();
    //배열선언
    // let start_num: i32 = input.parse::<i32>().unwrap();
    let v2: Vec<i32> = numbers_arry
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let result1 = v2.iter().max();
    let result2 = v2.iter().min();
    // for i in 0..start_num {
    //     (output, "{:?}{:?}", result1, result2).unwrap();
    // }
    // print!("{}", output);
    let mut result3 = 0;
    let mut result4 = 0;
    match result1 {
        // 분할이 유효 함
        Some(x) => result3 = *x,
        // 부서가 잘못되었습니다.
        None => println!("Cannot divide by 0"),
    }
    match result2 {
        // 분할이 유효 함
        Some(x) => result4 = *x,
        // 부서가 잘못되었습니다.
        None => println!("Cannot divide by 0"),
    }
    println!("{} {}", result4, result3);
}
//아직못품
