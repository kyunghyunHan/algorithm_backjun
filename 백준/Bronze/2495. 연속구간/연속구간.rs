fn main() {
    let mut input = String::new();

    // 3개의 숫자 입력 받기
    for _ in 0..3 {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let number = input.trim();

        let mut max_len = 1;  // 최소 1부터 시작
        let mut current_len = 1;

        // 8자리 숫자에서 연속된 숫자 길이 계산
        for i in 1..number.len() {
            if number.chars().nth(i) == number.chars().nth(i - 1) {
                current_len += 1;
            } else {
                max_len = max_len.max(current_len);
                current_len = 1;
            }
        }

        // 마지막 구간 체크
        max_len = max_len.max(current_len);

        println!("{}", max_len);
    }
}
