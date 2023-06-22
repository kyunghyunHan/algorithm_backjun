use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let nums= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<usize>>();
  
    writeln!(writer,"{}",((nums[1]/nums[0])*3)*nums[2]).unwrap();

}