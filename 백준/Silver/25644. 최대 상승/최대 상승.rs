use std::io::{BufReader, BufWriter, BufRead, Write, stdin, stdout};

fn main() {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    // 첫 번째 입력: n (주식 가격의 개수)
    let n = match input.next() {
        Some(Ok(line)) => line.parse::<usize>().unwrap(),
        _ => panic!("error"),
    };

    // 두 번째 입력: 주식 가격 리스트
    let stock_list = match input.next() {
        Some(Ok(line)) => line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>(),
        _ => panic!("error"),
    };

    // 최소 가격 및 최대 이익 계산
    let mut min_price = u32::MAX;
    let mut max_profit = 0;

    for &price in &stock_list {
        if price < min_price {
            min_price = price; // 현재까지의 최소 가격 갱신
        } else {
            max_profit = max_profit.max(price - min_price); // 최대 이익 갱신
        }
    }

    writeln!(writer, "{}", max_profit).unwrap();
}
