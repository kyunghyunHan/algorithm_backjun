use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let mut nh = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let n = nh[0];
        let mut h = nh[1];
        if let Some(Ok(line)) = input.next() {
            let mut v = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let mut cur= 0;
            let mut cur = 0;
            for i in 0..n as usize {
                cur += v[i];
                if cur >= h {
                    writeln!(writer, "{}", i + 1).unwrap();
                    return;
                }
            }

            // 루프를 다 돌고도 체력을 깎지 못했을 경우 -1 출력
            writeln!(writer, "-1").unwrap();
            /*
            카드의 수 n
            상대 체력은 h


            어떤 카드를 사용하여 공격하면 상대의 체력은 공격력 만큼 줄어듬

            상대를 최대한 늦게 죽여야함

             */
        }
    }
}
