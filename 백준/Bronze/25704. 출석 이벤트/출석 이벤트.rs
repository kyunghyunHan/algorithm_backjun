use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 입력
    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap(); // 도장 수
    let p: i32 = lines.next().unwrap().unwrap().parse().unwrap(); // 가격

    // 최대 할인가격 구하기
    let mut max_discount = 0;

    // 도장 갯수별 할인
    if n >= 5 {
        max_discount = max_discount.max(500);
    }

    if n >= 10 {
        max_discount = max_discount.max(p / 10);
    }

    if n >= 15 {
        max_discount = max_discount.max(2000);
    }

    if n >= 20 {
        max_discount = max_discount.max(p / 4);
    }

    // 할인금액이 더 크면 0원
    println!("{}", if p <= max_discount { 0 } else { p - max_discount });
}