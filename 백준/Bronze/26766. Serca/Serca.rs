use std::io::{self, BufRead, Write};

fn print_heart(output: &mut dyn Write) {
    writeln!(output, " @@@   @@@ ").unwrap();
    writeln!(output, "@   @ @   @").unwrap();
    writeln!(output, "@    @    @").unwrap();
    writeln!(output, "@         @").unwrap();
    writeln!(output, " @       @ ").unwrap();
    writeln!(output, "  @     @  ").unwrap();
    writeln!(output, "   @   @   ").unwrap();
    writeln!(output, "    @ @    ").unwrap();
    writeln!(output, "     @     ").unwrap();
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());

    for _ in 0..n {
        print_heart(&mut output);
    }

    output.flush().unwrap();
}
