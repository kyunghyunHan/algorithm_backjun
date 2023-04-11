use std::collections::HashSet;
use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
//Hashset은 빠른 검색 속도
fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    // Read the first line of input
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n1 = buffer.trim().parse::<usize>().unwrap();

    // Read the second line of input
    let mut buffer2 = String::new();
    reader.read_line(&mut buffer2).unwrap();
    let cards: HashSet<i32> = buffer2
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read the third line of input
    let mut buffer3 = String::new();
    reader.read_line(&mut buffer3).unwrap();
    let n2 = buffer3.trim().parse::<usize>().unwrap();

    // Read the fourth line of input
    let mut buffer4 = String::new();
    reader.read_line(&mut buffer4).unwrap();
    let cards2: Vec<i32> = buffer4
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Search for elements of cards2 in cards
    let mut results = vec![0; n2];
    for i in 0..n2 {
        if cards.contains(&cards2[i]) {
            results[i] = 1;
        }
    }

    // Write the results to standard output
    for i in results {
        write!(writer, "{} ", i).unwrap();
    }
}