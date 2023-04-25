use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let reader = BufReader::new(stdin());
    let mut writer = BufWriter::new(stdout());
    let mut lines = reader.lines();
    
    // 첫 번째 입력값 처리
    let a = lines.next().unwrap().unwrap().parse::<isize>().unwrap();
    
    // 두 번째 입력값 처리
    let b = lines.next().unwrap().unwrap().parse::<isize>().unwrap();
    let c = lines.next().unwrap().unwrap().parse::<isize>().unwrap();

    let d = lines.next().unwrap().unwrap().parse::<isize>().unwrap();
    let e = lines.next().unwrap().unwrap().parse::<isize>().unwrap();

    // 결과 계산 및 출력
    let result = a+b+c+d+e;
    writeln!(writer, "{}", result).unwrap();
}