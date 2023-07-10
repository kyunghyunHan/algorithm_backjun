use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut input= String::new();
let mut writer= BufWriter::new(stdout().lock());
reader.read_line(&mut input).unwrap();
let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

 let mut v= vec![0,0,0];
 


if 7000*nums[1]<= 1000 *nums[0]{
    v[0] = 7000 * nums[1];
}else if 3500*nums[1]<= 1000 *nums[0]{
    v[1] = 3500 * nums[1];
}else if 1750*nums[1]<= 1000 *nums[0]{
    v[2] = 1750 * nums[1];
}

v.sort();

writeln!(writer,"{}",v[2]).unwrap();
}
