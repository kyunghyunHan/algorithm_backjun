use std::{
    cmp::min,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let v: Vec<usize> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let a = v[0];
        let b = v[1];
        let c = v[2];
        let d = v[3];

        let mininum_mice = a + b + c + d - v.iter().min().unwrap() + 1;

        writeln!(writer, "{}", mininum_mice).unwrap();
    }
    writer.flush().unwrap();
}
