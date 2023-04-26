use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input"); // 표준 입력에서 입력값을 읽음

    let n: i32 = n.trim().parse().expect("Failed to parse input"); // 입력값을 정수로 변환

    let result1 = n * 78 / 100;
    let result2 = n * 8 / 10 + (n * 156 / 1000);

    println!("{} {}", result1, result2); // 결과 출력
}