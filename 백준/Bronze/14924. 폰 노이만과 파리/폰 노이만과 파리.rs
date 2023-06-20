use std::io::{BufReader,BufRead,BufWriter,stdin,stdout,Write};

fn main(){
    let mut reader=BufReader::new(stdin().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let mut writer= BufWriter::new(stdout().lock());
let nums= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<usize>>();
writeln!(writer,"{}",((nums[2]/2)/nums[0])*nums[1]).unwrap();
}