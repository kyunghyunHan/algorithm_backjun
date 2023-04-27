use std::io::{self, BufRead, BufWriter, Write};

fn hanoi(n: i32, start: i32, to: i32, bypass: i32, writer: &mut BufWriter<io::StdoutLock>) {
    if n == 1 {
        writeln!(writer, "{} {}", start, to).unwrap();
    } else {
        hanoi(n - 1, start, bypass, to, writer);
        writeln!(writer, "{} {}", start, to).unwrap();
        hanoi(n - 1, bypass, to, start, writer);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().next().unwrap().unwrap();
    let num: i32 = input.trim().parse().unwrap();

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{}", (1 << num) - 1).unwrap();
    hanoi(num, 1, 3, 2, &mut writer);
    writer.flush().unwrap();
}
