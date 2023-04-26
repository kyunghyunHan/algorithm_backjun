use std::io::{BufWriter, BufReader, BufRead, Write, stdin, stdout};
use std::cmp;

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
 
   

    let mut input = String::new();
      reader.read_line(&mut input).unwrap();
      let mut num = input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
      let a: usize= num.next().unwrap();
      let b: usize= num.next().unwrap();
      let c: usize= num.next().unwrap();
      
      let mut v= [a,b,c];
      v.sort(); 
      writeln!(writer,"{}",v[1]).unwrap();

    

    
}
