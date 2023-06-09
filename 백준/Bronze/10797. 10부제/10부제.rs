use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
let mut writer= BufWriter::new(stdout().lock());
    let n:i32= input.trim().parse().unwrap();
let mut result=0;
    input.clear();
    reader.read_line(&mut input).unwrap();
    let nums=input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<usize>>();

    for i in nums{
        if i as i32==n as i32{
            result+=1;
        }
    }

writeln!(writer,"{}",result).unwrap();
}