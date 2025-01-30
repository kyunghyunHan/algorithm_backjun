fn main() {
    let t: usize = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    for case_num in 1..=t {
        let n: usize = {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().parse().unwrap()
        };
        
        let mut heights: Vec<usize> = {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
        };
        
        let mut to_destroy = 0;
        let mut max_height = heights[0];

        // 왼쪽에서 오른쪽으로 지나가면서 높이를 확인
        for i in 1..n {
            if heights[i] <= max_height {
                to_destroy += 1;  // 이 집을 파괴해야 함
            } else {
                max_height = heights[i];  // 호수를 볼 수 있는 높이로 업데이트
            }
        }

        println!("Case #{}: {}", case_num, to_destroy);
    }
}
