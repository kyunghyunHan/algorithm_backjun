use core::num;
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse().unwrap();
    for i in 0..n{
        
       input.clear();
       reader.read_line(&mut input).unwrap();
        let n:u32= input.trim().parse().unwrap();
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
        let result:usize=nums.iter().sum();
        writeln!(writer,"{}",result).unwrap();
    }
}