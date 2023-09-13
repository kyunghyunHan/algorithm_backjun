use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    
    let mut nums= input.trim().split_ascii_whitespace().map(|x|x.parse::<i32>().unwrap());
    let a= nums.next().unwrap();
    let i = nums.next().unwrap();
    
    writeln!(writer,"{}",a*(i-1)+1).unwrap();
    
}