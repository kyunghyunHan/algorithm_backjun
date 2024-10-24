use std::{
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
    string,
};
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    let t = input.next().unwrap().unwrap().parse::<usize>().unwrap();
    for i in 0..t {
        let n = input.next().unwrap().unwrap().trim().to_string();
        let n2 = n.parse::<usize>().unwrap().pow(2).to_string();
        let niter = n2.chars().rev().take(n.len()).collect::<Vec<char>>();
        let last = niter.iter().rev().map(|x| *x).collect::<Vec<char>>();
        
        if n.chars().collect::<Vec<char>>() == last {
            writeln!(writer, "{}", "YES").unwrap();
        } else {
            writeln!(writer, "{}", "NO").unwrap();
        }
    }
    writer.flush().unwrap();
}
