use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::prelude::*;
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();

    let  nums= input.trim().split_ascii_whitespace().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    if nums == [1,2,3,4,5,6,7,8]{
       writeln!(writer,"{}","ascending").unwrap();
    }else if nums ==[8,7,6,5,4,3,2,1]{
       writeln!(writer,"{}","descending").unwrap();
    }else{
        writeln!(writer,"{}","mixed").unwrap();

    }
}