use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();

    reader.read_line(&mut input).unwrap();

    let mut nums:Vec<usize>=input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    
    nums.sort();
    writeln!(writer,"{}",nums[0] *nums[2]).unwrap();

}