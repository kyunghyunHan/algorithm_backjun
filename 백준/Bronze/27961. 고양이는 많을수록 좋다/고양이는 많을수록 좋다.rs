use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let mut n = line.trim().parse::<u64>().unwrap();
        let mut count = 0;
        let mut a = 0;

        while a < n {
            if a == 0 {
                a += 1; //처음에는 1마리 고양이 추가
            } else {
                a *= 2; //이후에는 수를 2배로증가
            }
            count += 1; //한번의 행동이므로 카운터 증가
        }

        writeln!(writer, "{}", count).unwrap();
    }

    writer.flush().unwrap();
}
