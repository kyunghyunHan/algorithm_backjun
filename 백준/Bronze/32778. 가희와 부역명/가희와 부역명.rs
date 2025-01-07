use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    // 입력을 한 줄 읽기
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let input = input.trim(); // 앞뒤 공백 제거

    // 괄호를 기준으로 역명과 부역명 나누기
    if let Some(open_idx) = input.find('(') {
        if let Some(close_idx) = input.find(')') {
            let station_name = &input[..open_idx].trim(); // 괄호 전 역명
            let sub_station_name = &input[open_idx + 1..close_idx].trim(); // 괄호 안 부역명
            writeln!(writer, "{}", station_name).unwrap();
            writeln!(writer, "{}", sub_station_name).unwrap();
        }
    } else {
        // 부역명이 없는 경우
        writeln!(writer, "{}", input).unwrap();
        writeln!(writer, "-").unwrap();
    }

    writer.flush().unwrap();
}
