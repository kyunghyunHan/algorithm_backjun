use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    let n = match input.next() {
        Some(Ok(line)) => line.trim().parse::<f64>().expect("Invalid input for n"),
        _ => panic!("Failed to read input for n"),
    };
    let x = match input.next() {
        Some(Ok(line)) => line.trim().parse::<f64>().expect("Invalid input for x"),
        _ => panic!("Failed to read input for x"),
    };

    // 손해를 본 후의 금액 계산
    let current = n * (1.0 - x / 100.0);

    // 복구를 위해 필요한 상승 비율 계산
    let percentage_increase = ((n / current) - 1.0) * 100.0;


    // 출력
    writeln!(writer, "{:.6}", percentage_increase).unwrap();
    writer.flush().unwrap();
}
