use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap(); // n 입력 (사용 안 하지만 보관)

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut high = 0; // 제일 높은 봉우리
    let mut cnt = 0; // 사냥꾼이 잡을 수 있는 수
    let mut total = Vec::new(); // 전체 저장

    for &i in &arr {
        if i > high {
            high = i; // 최댓값 갱신
            cnt = 0;  // 초기화
        } else {
            cnt += 1; // 용을 사냥할 수 있음
        }
        total.push(cnt); // 배열에 추가
    }

    if let Some(max_value) = total.iter().max() {
        println!("{}", max_value); // 가장 큰 값 출력
    }
}
