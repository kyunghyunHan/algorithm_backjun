use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums:Vec<i128>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();  

    writeln!(writer,"{} {}",(nums[0]+1)*2, (nums[0]+1)*3).unwrap();

}