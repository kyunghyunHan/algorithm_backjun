use std::io::{stdin, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();
    //세번째는 각  항목이 입력받은 순서
    let mut pairs: Vec<(i32, String, usize)> = Vec::new();
    for (i, line) in reader.lines().take(n).enumerate() {
        let buffer = line.unwrap();
        let mut nums = buffer.trim().split_whitespace();
        let a = nums.next().unwrap().parse::<i32>().unwrap();
        let b = nums.next().unwrap().to_string();
        pairs.push((a, b, i));
    }

    pairs.sort_by(|a, b| {
        if a.0 == b.0 {
            a.2.cmp(&b.2)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for (a, b, _) in pairs {
        write!(writer, "{} {}\n", a, b).unwrap();
    }
}