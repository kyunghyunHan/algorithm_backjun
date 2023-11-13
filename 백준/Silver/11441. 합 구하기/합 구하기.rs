use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();
    let nums: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();
    reader.read_line(&mut input).unwrap();
    let m: usize = input.trim().parse().unwrap();

    let mut prefix_sum = vec![0; n + 1];

    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + nums[i];
    }

    for _ in 0..m {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let ij: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let result = prefix_sum[ij[1]] - prefix_sum[ij[0] - 1];
        writeln!(writer, "{}", result).unwrap();
    }

    writer.flush().unwrap();
}
