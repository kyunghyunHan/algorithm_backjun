use std::io::{self, BufRead,BufReader,BufWriter,Write,stdin,stdout};

fn main() {
   let mut reader= BufReader::new(stdin().lock());

   let mut input= String::new();
   reader.read_line(&mut input).unwrap();

   let n= input.trim().parse::<usize>().unwrap();
let mut re= 1;
let mut writer= BufWriter::new(stdout().lock());
let mut piece= 1;
   for i in 0..n{
      if i%2 !=0{
        re+=1;
      }
      piece+=re;

   }
   writeln!(writer,"{}",piece).unwrap();
}
