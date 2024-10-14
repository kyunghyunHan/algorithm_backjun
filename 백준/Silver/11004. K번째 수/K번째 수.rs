use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write}; //원형큐
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut wirter = BufWriter::new(stdout().lock());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut input = line
        .trim()
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>);
    let n = input.next().unwrap();
    let k = input.next().unwrap();
    line.clear();
    reader.read_line(&mut line).unwrap();

    let mut a = line
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    a.sort();
    writeln!(wirter, "{}", &a[k as usize - 1]).unwrap();
    wirter.flush().unwrap();
}
