use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut count = 0;

    for i in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s = input.trim();
        if s.contains("OI"){
            count+=1;
        }else if  s.contains("01"){
            count+=1;
        }
        
    }
    writeln!(writer,"{}",count).unwrap();
}
