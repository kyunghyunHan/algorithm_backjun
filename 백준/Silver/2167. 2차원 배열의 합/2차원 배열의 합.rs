use std::io::{BufRead, BufReader, BufWriter, Write, stdin, stdout};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();

    // 첫 줄: N, M 입력
    reader.read_line(&mut input).unwrap();
    let (n, m) = {
        let nm = input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        (nm[0], nm[1])
    };

    // 배열 입력
    let mut v = vec![vec![0; m]; n];
    for i in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let row = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        v[i] = row;
    }

    // 누적합 배열 만들기
    let mut sum = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            sum[i][j] = sum[i-1][j] + sum[i][j-1] - sum[i-1][j-1] + v[i-1][j-1];
        }
    }

    // K 입력
    input.clear();
    reader.read_line(&mut input).unwrap();
    let k = input.trim().parse::<usize>().unwrap();

    // K개의 쿼리 처리
    for _ in 0..k {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let query = input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let (i, j, x, y) = (query[0], query[1], query[2], query[3]);

        // 구간합 계산
        let result = sum[x][y] - sum[i-1][y] - sum[x][j-1] + sum[i-1][j-1];

        writeln!(writer, "{}", result).unwrap();
    }
}
