use std::{
    cmp,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let nc = input
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    let n = nc[0];
    let c = nc[1];
    let mut houses = Vec::new();
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let x = input.trim().parse::<usize>().unwrap();
        houses.push(x);
    }
    houses.sort();

    let mut start = 1;
    let mut end = houses[n  as usize- 1] - houses[0];
    let mut result = 0;

    while start <= end {
        let mid = (start + end) / 2;
        let mut count = 1;
        let mut prev_house = houses[0];

        for i in 1..n {
            if houses[i as usize] - prev_house >= mid {
                count += 1;
                prev_house = houses[i as usize];
            }
        }
        if count >= c {
            result = cmp::max(result, mid);
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    writeln!(writer, "{}", result).unwrap();
    writer.flush().unwrap();
}