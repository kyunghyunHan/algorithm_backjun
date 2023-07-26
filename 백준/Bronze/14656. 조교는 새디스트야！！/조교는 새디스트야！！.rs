use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader=BufReader::new(stdin().lock());

    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();

    let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let mut count= 0;
    for i in 1..=n{
  if        i!=nums[i-1]{
 count+=1;
        }
    }
    writeln!(writer,"{}",count).unwrap();
}