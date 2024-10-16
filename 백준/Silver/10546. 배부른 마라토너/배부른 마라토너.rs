use std::collections::HashMap;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write}; //원형큐

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut wirter = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut runner: HashMap<String, i32> = HashMap::new();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let name = input.trim().to_string();
        *runner.entry(name).or_insert(0) += 1;
    }
    for _ in 0..(n - 1) {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let name = input.trim().to_string();
        *runner.entry(name).or_insert(0) -= 1;
    }
    for (name, count) in runner {
        if count != 0 {
            writeln!(wirter, "{}", name).expect("Failed to write output");
            break;
        }
    }
    wirter.flush().unwrap();
}
