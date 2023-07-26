use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader=BufReader::new(stdin().lock());

    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();

    let half:f32= (n as f32)/2.0;
    let result:i32 = (half * half +0.5) as i32;

    writeln!(writer,"{}",result).unwrap();
}