use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let mut n:u64= input.trim().parse().unwrap();
    let mut cnt: u64= 0;
    let mut i =1;
     while i<=n {
        cnt += (n-i+1);

     i*=10;
     }

     writeln!(writer,"{}",cnt).unwrap();
    
}