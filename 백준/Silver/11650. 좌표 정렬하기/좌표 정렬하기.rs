use std::io::{stdin, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut pairs: Vec<(i32, i32)> = Vec::new();
    for _ in 0..n {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        let mut nums = buffer.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();
        pairs.push((a, b));
    }

    pairs.sort();
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for (a, b) in pairs {
        write!(writer, "{} {}\n", a, b).unwrap();
    }
}