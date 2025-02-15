use std::{
    cmp::max,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    let mut sum = 0;
    let mut best = 0;

    for i in 1..=10 {
        if let Some(Ok(line)) = input.next() {
            let n = line.parse::<i32>().unwrap();
            let new_sum = sum + n;

            if (new_sum - 100).abs() < (sum - 100).abs() {
                best = new_sum;
            } else if (new_sum - 100).abs() == (sum - 100).abs() {
                best = new_sum.max(sum);
            }

            sum = new_sum;
        }
    }

    writeln!(writer,"{}",best).unwrap();
}
