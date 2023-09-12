use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let num = input.trim().parse::<i32>().unwrap();
    for i in 0..num{
        for j in 0..=i{
            write!(writer,"{}","*").unwrap();
        }
        writeln!(writer).unwrap();
    }
}