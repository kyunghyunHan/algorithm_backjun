use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_ascii_whitespace().map(|x|x.parse::<u64>().unwrap());
    let n= nums.next().unwrap();
    let b= nums.next().unwrap();
    let mut sum=1 ;

    for i in 1..=b{
        sum+=1u64 <<i
    }

    if sum>=n {
        writeln!(writer,"{}","yes").unwrap();
    }else{
        writeln!(writer,"{}","no").unwrap();

    }
    writer.flush().unwrap();
}