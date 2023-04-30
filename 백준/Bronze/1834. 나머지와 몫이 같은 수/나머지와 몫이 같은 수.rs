use std::{io::{BufRead,BufWriter,BufReader,Write,stdin,stdout}, usize};

fn main(){


    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse::<usize>().unwrap();
     let mut sum = 0;
    for i in 0..n{
       sum+=i*(n+1);
    }

    writeln!(writer,"{}",sum).unwrap();


}