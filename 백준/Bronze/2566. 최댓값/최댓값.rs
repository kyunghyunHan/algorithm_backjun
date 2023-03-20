use std::io;


fn main() {
    let mut arr = [[0; 9]; 9];
    let mut max = 0;
    let mut idx_x = 0;
    let mut idx_y = 0;

    // 배열 입력 받기
    for i in 0..9 {
        let mut input_a = String::new();
        io::stdin().read_line(&mut input_a).unwrap();
        let v: Vec<usize> = input_a
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for j in 0..9 {
            arr[i][j] = v[j];
        }
    }

    // 최댓값과 그 위치 찾기
    for i in 0..9 {
        for j in 0..9 {
            if max < arr[i][j] {
                max = arr[i][j];
                idx_x = i;
                idx_y = j;
            }
        }
    }

    // 결과 출력
    println!("{}", max);
    println!("{} {}", idx_x+1, idx_y+1);
}