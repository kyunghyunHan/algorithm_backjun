use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // 첫 번째 줄 입력
    let (a, b) = {
        let line = input.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        (nums.next().unwrap(), nums.next().unwrap())
    };

    // 두 번째 줄 입력
    let (c, d) = {
        let line = input.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        (nums.next().unwrap(), nums.next().unwrap())
    };

    // 플레이어 계산
    let mut answer = (a + b - 1) % 4;
    answer = (answer + c + d - 1) % 4;

    println!("{}", answer + 1);
}
