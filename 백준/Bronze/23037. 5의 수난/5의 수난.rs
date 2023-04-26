use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력을 읽을 수 없습니다.");
    let n: i32 = input.trim().parse().expect("유효한 숫자를 입력하세요.");

    // 각 자릿수의 다섯제곱의 합 계산
    let mut sum = 0;
    let mut num = n;
    while num > 0 {
        let digit = num % 10;
        sum += digit.pow(5);
        num /= 10;
    }

    // 결과 출력
    println!("{}", sum);
}
