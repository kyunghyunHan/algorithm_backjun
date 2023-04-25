use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout());
    let mut lines = reader.lines();
    
    // 첫 번째 입력값 처리
    let a = lines.next().unwrap().unwrap().parse::<isize>().unwrap();
   
    writeln!(writer, "{}", a-1946).unwrap();
}