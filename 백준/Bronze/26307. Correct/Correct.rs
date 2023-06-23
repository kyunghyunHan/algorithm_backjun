use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();


    reader.read_line(&mut input).unwrap();

    let nums: Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<usize>>();


    let hh= nums[0];
    let mm = nums[1];
let mut result= 0;
    result= ((hh-9)*60)+mm;

    writeln!(writer,"{}",result).unwrap();
}