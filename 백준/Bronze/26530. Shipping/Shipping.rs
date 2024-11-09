use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    let n: usize = input.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        let x: usize = input.next().unwrap().unwrap().trim().parse().unwrap();

        let mut total_money = 0.0;

        for _ in 0..x {
            if let Some(Ok(line)) = input.next() {
                let parts: Vec<&str> = line.split_whitespace().collect();

                let quantity: i32 = parts[1].parse().unwrap();
                let unit_price: f64 = parts[2].parse().unwrap();

                total_money += quantity as f64 * unit_price;
            }
        }

        writeln!(writer, "${:.2}", total_money).unwrap();
        writer.flush().unwrap();
    }
}
