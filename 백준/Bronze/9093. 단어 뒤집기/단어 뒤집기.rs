use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
   let n= input.trim().parse().unwrap();

   for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();

      let mut nums:Vec<&str>= input.trim().split_whitespace().map(|x|x).collect();
 
    for i in  nums{
        let reversed: String = i.chars().rev().collect();
        write!(writer,"{} ",reversed).unwrap();
    }
    writeln!(writer,"").unwrap();
   }
  
}