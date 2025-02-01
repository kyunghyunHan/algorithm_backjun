use std::{
    cmp::max,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.trim().parse::<usize>().unwrap();
        let mut dp = [[0u64; 501]; 501];

        for i in 1..=n {
            let v = input
                .next()
                .unwrap()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            for j in 1..i + 1 {
                dp[i][j] = v[j - 1] + max(dp[i - 1][j - 1], dp[i - 1][j])
            }
        }
        let maxs = dp[n].iter().max().unwrap();

        writeln!(writer, "{:?}", maxs).unwrap();
    }
    writer.flush().unwrap();
}
