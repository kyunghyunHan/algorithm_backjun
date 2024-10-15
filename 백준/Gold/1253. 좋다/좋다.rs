use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write}; //원형큐
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut wirter = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();

    let mut a = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    a.sort();
    let mut ans = 0;
    for k in 0..n {
        let find = a[k as usize];
        let mut i = 0;
        let mut j = n - 1;
        while i < j {
            if a[i as usize] + a[j as usize] == find {
                if i != k && j != k {
                    ans += 1;
                    break;
                } else if i == k {
                    i += 1;
                } else if j == k {
                    j -= 1;
                }
            } else if a[i as usize] + a[j as usize] < find {
                i += 1;
            } else {
                j -= 1;
            }
        }
    }
    writeln!(wirter, "{}", ans).unwrap();
    wirter.flush().unwrap();
}
