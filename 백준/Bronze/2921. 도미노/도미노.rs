use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();

    reader.read_line(&mut input).unwrap();

    let mut n :usize=  input.trim().parse().unwrap();
    let mut res:f64= 0.0;

    for i in 1..n+1{
       res+=1.5*i as f64*(i as f64+1.0)
    }
    writeln!(writer,"{}",res as i128).unwrap();
}