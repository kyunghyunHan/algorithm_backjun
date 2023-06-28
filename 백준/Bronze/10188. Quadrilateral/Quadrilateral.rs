use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
   
    let mut writet= BufWriter::new(stdout().lock());
   
    reader.read_line(&mut input).unwrap();  
    let mut n:u64= input.trim().parse().unwrap();
     
    for i in 0..n{
      input.clear();
      reader.read_line(&mut input).unwrap();

      let mut nums:Vec<u64>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

      for i in 0..nums[1]{
        for j in 0..nums[0]{
          write!(writet,"{}","X").unwrap();
        }
        writeln!(writet,"").unwrap();
      }

      writeln!(writet).unwrap();
    }
}
