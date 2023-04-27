use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};

fn main(){


    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
    let mut result= 0;
    let mut writer= BufWriter::new(stdout().lock());
    for i in 0..6{
        let n= nums.next().unwrap();
        result+=n;
    }


    writeln!(writer,"{}",result*5).unwrap();

}