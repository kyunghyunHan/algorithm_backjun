use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let v = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let (a, b, c, m) = (v[0], v[1], v[2], v[3]);

        if a > m {
            // 한 시간 일했을 때 피로도가 최대치를 넘으면 아예 일을 시작할 수 없음
            writeln!(writer, "0").unwrap();
            return;
        }

        let mut fatigue = 0;
        let mut work_done = 0;

        for _ in 0..24 {
            if fatigue + a <= m {
                // 일을 할 수 있는 경우
                fatigue += a;
                work_done += b;
            } else {
                // 쉬어서 피로도를 감소
                fatigue -= c;
                if fatigue < 0 {
                    fatigue = 0;
                }
            }
        }

        writeln!(writer, "{}", work_done).unwrap();
    }
}
