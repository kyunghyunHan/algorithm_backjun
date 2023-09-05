use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());

    let mut writer= BufWriter::new(stdout().lock());

    let  mut input= String::new();
    reader.read_line(&mut input).unwrap();

    let mut nums:Vec<i64>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

    let a=100 - nums[0];
    let b=100 - nums[1];
    let c = 100 -(a+b);
    let d= a*b ;
    let q= d/100;
    let r = d%100;

    writeln!(writer,"{} {} {} {} {} {}", a, b, c, d, q, r).unwrap();

    writeln!(writer,"{} {}",c+q,r).unwrap();

}