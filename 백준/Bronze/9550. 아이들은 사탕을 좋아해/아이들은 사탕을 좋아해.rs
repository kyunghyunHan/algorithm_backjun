use std::io::{stdin, BufRead, BufReader, Write, BufWriter};

pub fn main() {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(std::io::stdout().lock());
    let mut input = reader.lines();
    
    let t = input.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    for _ in 0..t {
        if let Some(Ok(line)) = input.next() {
            let nk = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            let n = nk[0];
            let k = nk[1];

            if let Some(Ok(line)) = input.next() {
                let candy: Vec<i32> = line
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                
                let result: i32 = candy.iter().map(|&c| c / k).sum();
                writeln!(writer, "{}", result).unwrap();
            }
        }
    }

    writer.flush().unwrap();
}
