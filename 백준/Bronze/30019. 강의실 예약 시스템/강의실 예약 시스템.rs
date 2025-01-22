use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let nm = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let n = nm[0];
        let m = nm[1];
        let mut v = vec![0; n + 1]; // 1번 강의실부터 n번 강의실까지 (0번은 사용 안 함)
        for _ in 0..m {
            if let Some(Ok(line)) = input.next() {
                let kv = line
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                let k = kv[0]; // 강의실 번호
                let s = kv[1]; // 시작 시각
                let e = kv[2]; // 끝 시각
                if s >= v[k] {
                    v[k] = e;
                    writeln!(writer, "YES").unwrap();
                } else {
                    // 예약을 거부
                    writeln!(writer, "NO").unwrap();
                }
            }
        }
    }
    writer.flush().unwrap();
}
