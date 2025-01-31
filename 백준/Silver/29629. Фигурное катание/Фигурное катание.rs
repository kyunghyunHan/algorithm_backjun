use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let v = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        let n: usize = v[0]; 
        let m: usize = v[1]; 
        let x: usize = v[2]; 
        let y: usize = v[3]; 

        let mut elements = vec![];

        for _ in 0..n {
            if let Some(Ok(line)) = input.next() {
                let nums: Vec<usize> = line
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                elements.push((nums[0], nums[1]));  // (b_i, a_i)
            }
        }

        const INF: usize = usize::MAX;
        let mut dp = vec![INF; m + 1];
        dp[0] = 0;

        for &(b_i, a_i) in &elements {
            let mut new_dp = vec![INF; m + 1];
            for j in 0..=m {
                if dp[j] != INF {
                    for score in 0..=a_i {
                        if score >= b_i && j + score <= m {
                            let cost = if score == 0 {
                                0
                            } else if score < 10 {
                                x
                            } else {
                                y
                            };
                            new_dp[j + score] = new_dp[j + score].min(dp[j] + cost);
                        }
                    }
                }
            }
            dp = new_dp;
        }

        if dp[m] == INF {
            writeln!(writer, "-1").unwrap();
        } else {
            writeln!(writer, "{}", dp[m]).unwrap();
        }
    }

    writer.flush().unwrap();
}