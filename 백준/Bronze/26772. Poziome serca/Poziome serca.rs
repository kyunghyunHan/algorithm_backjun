use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();
        let heart_pattern = [
            " @@@   @@@ ",
            "@   @ @   @",
            "@    @    @",
            "@         @",
            " @       @ ",
            "  @     @  ",
            "   @   @   ",
            "    @ @    ",
            "     @     ",
        ];
        let height = heart_pattern.len();
        for i in 0..height {
            for j in 0..n {
                write!(writer,"{}", heart_pattern[i]).unwrap();
                if j < n - 1 {
                    write!(writer," ").unwrap();
                }
            }
            writeln!(writer).unwrap();
        }
    }
}
