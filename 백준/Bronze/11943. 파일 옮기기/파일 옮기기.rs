use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){

    

let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();
reader.read_line(&mut input).unwrap();


let nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

input.clear();

reader.read_line(&mut input).unwrap();
let nums2= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();


let a= nums[0]+nums2[1];
let b= nums[1]+nums2[0];

writeln!(writer,"{}",usize::min(a,b)).unwrap();
}
