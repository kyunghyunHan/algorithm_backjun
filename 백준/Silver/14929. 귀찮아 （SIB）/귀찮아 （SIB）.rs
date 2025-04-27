use std::io::{BufRead, BufReader, BufWriter, Write, stdin, stdout};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        if let Some(Ok(line)) = input.next() {
            let x = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let mut prefix_sum = vec![0; n + 1];
            for i in 0..n {
                prefix_sum[i + 1] = prefix_sum[i] + x[i];
            }

            let mut answer: i64 = 0; // answer는 커질 수 있으니까 i64로
            for i in 0..n {
                let standard = x[i] as i64;
                let calculated_sum = (prefix_sum[n] - prefix_sum[i + 1]) as i64;
                answer += standard * calculated_sum;
            }
            writeln!(writer, "{}", answer).unwrap();
        }
    }
    writer.flush().unwrap();
}
