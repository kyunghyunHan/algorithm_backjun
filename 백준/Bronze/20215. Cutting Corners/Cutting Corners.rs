use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let (w, h) = {
            let wh = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<f64>>();
            (wh[0], wh[1])
        };

        let recangle_cut = w + h;
        let diagonal_cut = (w * w + h * h).sqrt();
        writeln!(writer, "{:.9}", recangle_cut - diagonal_cut).unwrap();
    }
    writer.flush().unwrap();
}
