use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let (Some(Ok(line))) = input.next() {
        let n = line.trim().parse::<usize>().unwrap();
        if let Some(Ok(line)) = input.next() {
            let s1= line.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<usize>().unwrap();
            if let Some(Ok(line)) = input.next() {
                let s2 = line.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<usize>().unwrap();
                let result = s1.min(s2);
                writeln!(writer,"{}",result).unwrap();
            } else {
                writeln!(writer, "{}", "ERR").unwrap();
            }
        } else {
            writeln!(writer, "{}", "ERR").unwrap();
        }
    } else {
        writeln!(writer, "{}", "ERR").unwrap();
    }
    writer.flush().unwrap();
}
