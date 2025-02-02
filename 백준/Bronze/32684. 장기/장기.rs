use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let cocjr = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        if let Some(Ok(line)) = input.next() {
            let ekwoo = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<f32>>();

            let cocjr_score = cocjr[0] * 13
                + cocjr[1] * 7
                + cocjr[2] * 5
                + cocjr[3] * 3
                + cocjr[4] * 3
                + cocjr[5] * 2;

            let ekwoo_score = ekwoo[0] * 13.
                + ekwoo[1] * 7.
                + ekwoo[2] * 5.
                + ekwoo[3] * 3.
                + ekwoo[4]* 3.
                + ekwoo[5] * 2.
                + 1.5;

            if cocjr_score as f32 > ekwoo_score {
                writeln!(writer, "{}", "cocjr0208").unwrap();
            } else {
                writeln!(writer, "{}", "ekwoo").unwrap();
            }
        }
    }
    writer.flush().unwrap();
}
