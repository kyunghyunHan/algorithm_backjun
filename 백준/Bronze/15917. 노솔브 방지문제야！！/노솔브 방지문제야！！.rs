use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};


fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input ).unwrap();
    let q: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..q {
        input.clear();
        reader.read_line(&mut input ).unwrap();
        let a: i32 = input.trim().parse().expect("Invalid input");

        if a & -a == a {
            writeln!(writer,"1").unwrap();
        } else {
            writeln!(writer,"0").unwrap();
        }
    }
}
