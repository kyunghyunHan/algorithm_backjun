use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
    let mut case_number = 0;

    loop {
        case_number+=1;
        if let Some(Ok(line)) = input.next() {
            let v = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<f32>().unwrap())
                .collect::<Vec<f32>>();
            let n = v[0] as usize;
            if n == 0 {
                break;
            }

            let d = if n % 2 == 0 {
                (v[n / 2] + v[n / 2 + 1]) / 2.0
            } else {
                v[(n + 1) / 2]
            };
            writeln!(writer, "Case {}: {:.1}", case_number, d).unwrap();
        }
    }
    writer.flush().unwrap();
}
