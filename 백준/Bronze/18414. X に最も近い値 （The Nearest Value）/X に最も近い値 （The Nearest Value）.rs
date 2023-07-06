use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut writer= BufWriter::new(stdout().lock());

    let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    


    let x= nums[0];
    let l= nums[1];
    let r=nums[2];

    if x<l {
        writeln!(writer,"{}",l).unwrap();
    }else if x >r {
        writeln!(writer,"{}",r).unwrap();
    }else {
        writeln!(writer,"{}",x).unwrap();
    }
}