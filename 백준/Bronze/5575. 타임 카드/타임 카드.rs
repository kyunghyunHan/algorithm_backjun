use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    for i in 0..3 {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let time: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (sh, sm, ss, eh, em, es) = (time[0], time[1], time[2], time[3], time[4], time[5]);
        let start = (sh * 3600) + (sm * 60) + ss;
        let end = (eh * 3600) + (em * 60) + es;
        let t = end - start;
        let h = t/3600;
        let m = (t % 3600)/60;
        let s = (t % 3600) % 60;
        writeln!(writer,"{h} {m} {s}").unwrap()
    }
    writer.flush().unwrap();
}
