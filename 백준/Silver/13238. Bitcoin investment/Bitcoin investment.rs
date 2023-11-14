use std::{io::{BufRead,BufReader,BufWriter,Write,stdin,stdout}, iter};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let n:u64= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut max_value= 0;
    let  nums= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<u32>>();
    for  i in 0..nums.len(){
            let value= nums[i..].iter().max().unwrap();
            max_value= u32::max(max_value,value-nums[i])
    }
    writeln!(writer,"{}",max_value).unwrap();
}