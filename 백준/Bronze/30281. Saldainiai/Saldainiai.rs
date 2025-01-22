use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    
    // Read N
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    
    // Read numbers, handling potential multiple lines
    let mut numbers = Vec::new();
    while numbers.len() < n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        numbers.extend(
            input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
        );
    }
    
    // Calculate sum and find minimum odd number if needed
    let mut answer = numbers.iter().sum::<i64>();
    if answer % 2 == 1 {
        // Find minimum odd number
        let min_odd = numbers.iter()
            .filter(|&&x| x % 2 == 1)
            .min()
            .unwrap();
        answer -= min_odd;
    }
    
    writeln!(writer, "{}", answer / 2).unwrap();
    writer.flush().unwrap();
}