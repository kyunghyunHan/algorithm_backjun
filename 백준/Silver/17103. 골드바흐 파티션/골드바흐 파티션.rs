use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
use std::collections::HashMap;

const MAX: usize = 1000000;

fn main() {
    let mut arr = [0; MAX + 1];

    for i in 2..=MAX {
        arr[i] = i;
    }

    for i in 2..=MAX {
        if arr[i] == 0 {
            continue;
        }
        for j in (i * i..=MAX).step_by(i) {
            arr[j] = 0;
        }
    }

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for _ in 0..t {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let n: usize = buffer.trim().parse().unwrap();

        let mut cnt = 0;
        for i in 2..n {
            if arr[n - i] + arr[i] == n {
                cnt += 1;
                if n - i == i {
                    cnt += 1;
                }
            }
        }
        writeln!(writer,"{}", cnt / 2).unwrap();
    }
}