use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut writer= BufWriter::new(stdout().lock());
    let mut nums:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let mut v= vec![];
    let mut count = 0;
    for i in 0..nums[0]{
        input.clear();
    reader.read_line(&mut input).unwrap();
    let n:i32 = input.trim().parse().unwrap();
      count+=n;
      v.push(n*nums[1]);
    }

    for i in v{
        writeln!(writer,"{}",i/count).unwrap();
        }
}