use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_ascii_whitespace().map(|x|x.to_string());

    let a:i64= nums.next().unwrap().chars().map(|x|x.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>().iter().sum();



    let b:i64= nums.next().unwrap().chars().map(|x|x.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>().iter().sum();


    writeln!(writer,"{}",a*b).unwrap();
    writer.flush().unwrap();
}