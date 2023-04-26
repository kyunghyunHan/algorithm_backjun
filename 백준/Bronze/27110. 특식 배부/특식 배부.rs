use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input"); // 표준 입력에서 입력값을 읽음

    let N: i32 = input.trim().parse().expect("Failed to parse input"); // 입력값을 정수로 변환

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input"); // 표준 입력에서 입력값을 읽음

    let chicken: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse input")) // 입력값을 정수로 변환하여 벡터에 저장
        .collect();

    let mut result = 0;

    for i in 0..3 {
        if chicken[i] <= N {
            result += chicken[i];
        } else {
            result += N;
        }
    }

    println!("{}", result); // 결과 출력
}
