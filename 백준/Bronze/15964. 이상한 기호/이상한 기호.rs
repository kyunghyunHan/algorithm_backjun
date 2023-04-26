use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_line(&mut buf)?;
    let mut st = buf.trim().split_whitespace();
    let a: i64 = st.next().unwrap().parse().unwrap();
    let b: i64 = st.next().unwrap().parse().unwrap();
    println!("{}", (a + b) * (a - b));
    Ok(())
}
