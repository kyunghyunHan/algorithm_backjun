use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut input= String::new();
let mut writer= BufWriter::new(stdout().lock());
reader.read_line(&mut input).unwrap();
let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
input.clear();
reader.read_line(&mut input).unwrap();

let mut days:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();


let mut result =days[0]*nums[1] +days[1]*nums[1];

for i in 0..nums[0]-1{
   if days[i]* nums[1]+days[i+1]*nums[1]<result {
    result = days[i]* nums[1]+days[i+1]*nums[1];
   }
}
writeln!(writer,"{}",result).unwrap();
}
