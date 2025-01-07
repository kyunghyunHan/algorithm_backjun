use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    match input.next() {
        Some(Ok(line)) => {
            let v = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u64>>();
            let r = v[0]; //초기보상
            let k = v[1]; //반감기 가격
            let m = v[2]; //특정 블록의 번호

            //k블록마다 보상이 절반으로 감소
            //1사토시 보다 작아지면

            let mut r = r;
            let mut iterations = m / k;

            while iterations > 0 && r > 0 {
                r >>= 1; // Perform bitwise right shift
                iterations -= 1;
            }

            writeln!(writer,"{}",r).unwrap();
        }
        _ => {}
    }
    writer.flush().unwrap();
}
