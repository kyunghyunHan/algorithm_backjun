use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    // 각 줄을 처리
    for line in stdin.lock().lines() {
        input = line.unwrap();
        
        // "0"을 입력받으면 종료
        if input.trim() == "0" {
            break;
        }

        let nums: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let n = nums[0] as usize; // 첫 번째 숫자는 N
        let mut result = Vec::new();
        
        // 연속된 중복을 제거하는 로직
        let mut prev = -1; // 초기값을 불가능한 숫자로 설정
        for i in 1..=n {
            let num = nums[i];
            if num != prev {
                result.push(num);
                prev = num;
            }
        }

        // 결과 출력
        for i in 0..result.len() {
            if i == result.len() - 1 {
                print!("{} $", result[i]);
            } else {
                print!("{} ", result[i]);
            }
        }
        println!(); // 줄 바꿈
    }
}
