use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut reader = reader.lines();

    match reader.next() {
        Some(Ok(line)) => {
            let n = line.parse::<usize>().unwrap();
            match reader.next() {
                Some(Ok(line)) => {
                    let v: Vec<i32> = line
                        .trim()
                        .split_whitespace()
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect();
                    let mut adp = [1u32; 100001];
                    let mut dedp = [1u32; 100001];

                    adp[0] = 1;
                    dedp[0] = 1;

                    for i in 1..n {
                        if v[i] == v[i - 1] {
                            adp[i] = adp[i - 1] + 1;
                            dedp[i] = dedp[i - 1] + 1;
                        } else if v[i] > v[i - 1] {
                            adp[i] = adp[i - 1] + 1;
                        } else {
                            dedp[i] = dedp[i - 1] + 1;
                        }
                    }

                    let adp_result = adp.iter().max().unwrap();
                    let dedp_result = dedp.iter().max().unwrap();

                    let result = *adp_result.max(dedp_result);

                    writeln!(writer,"{}",result).unwrap();

                }
                _ => {}
            }
        }
        _ => {}
    }
    writer.flush().unwrap();
}
