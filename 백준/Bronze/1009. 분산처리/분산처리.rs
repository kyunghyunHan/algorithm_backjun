use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:u64= input.trim().parse().unwrap();
    input.clear();
    for _ in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut nums = input.trim().split_ascii_whitespace().map(|x|x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let a= nums[0];
        let b= nums[1];
    
        let mut task = a;
        for _ in 1..b {
            task = (task * a) % 10; // Calculate the last digit using modulo 10
        }

        if task % 10 == 0 {
            writeln!(writer,"10").unwrap();
        } else {
            writeln!(writer,"{}", task % 10).unwrap()
            
        }
        
    }
}