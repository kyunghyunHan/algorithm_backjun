use std::io::{BufWriter, BufRead, BufReader, Write, stdin, stdout};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    // Read n from input
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    // Process n pairs of integers
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut num = input.split_whitespace().map(|x| x.parse::<isize>().unwrap());
        let n = num.next().unwrap();
        let m = num.next().unwrap();
        writeln!(writer, "{}", n + m).unwrap();
    }
}