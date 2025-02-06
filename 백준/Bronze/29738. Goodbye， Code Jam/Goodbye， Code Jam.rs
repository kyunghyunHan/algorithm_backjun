use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut wrier = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let t = line.parse::<usize>().unwrap();
        for i in 1..=t {
            if let Some(Ok(line)) = input.next() {
                let n = line.parse::<usize>().unwrap();
                if n <= 25 {
                    writeln!(wrier, "Case #{}: {}", i,"World Finals").unwrap();
                } else if n <= 1000 {
                    writeln!(wrier, "Case #{}: {}", i,"Round 3").unwrap();
                } else if n <= 4500 {
                    writeln!(wrier, "Case #{}: {}", i,"Round 2").unwrap();
                } else {
                    writeln!(wrier, "Case #{}: {}", i,"Round 1").unwrap();
                }
            }
        }
    }
    wrier.flush().unwrap();
}
