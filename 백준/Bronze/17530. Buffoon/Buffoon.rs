use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();

    // read n
    reader.read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();
    line.clear();

    // read votes
    let mut votes = vec![0; n];
    for i in 0..n {
        reader.read_line(&mut line).unwrap();
        votes[i] = line.trim().parse::<usize>().unwrap();
        line.clear();
    }

    // find winner
    let mut winner = 0;
    for i in 1..n {
        if votes[i] > votes[winner] {
            winner = i;
        }
    }

    // check if winner is first candidate
    let result = if winner == 0 { "S" } else { "N" };

    // print result
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", result).unwrap();
}
