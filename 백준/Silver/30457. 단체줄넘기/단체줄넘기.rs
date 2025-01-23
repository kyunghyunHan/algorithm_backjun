use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let mut n = line.trim().parse::<u64>().unwrap();
        let mut nums = [0; 1001];
        if let Some(Ok(line)) = input.next() {
            let mut heigh = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();
            /*
            n명의 학생들이 단체줄넘기
            학생들의 키를 나타냄
             */
            for i in heigh {
                nums[i] += 1;

                if nums[i] > 2 {
                    n -= 1;
                }
            }

            writeln!(writer, "{}", n).unwrap();
        }
    }

    writer.flush().unwrap();
}
