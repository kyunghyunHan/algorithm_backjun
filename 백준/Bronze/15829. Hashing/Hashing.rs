
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
   let mut reader= BufReader::new(stdin().lock());
   let mut writer= BufWriter::new(stdout().lock());
   let mut input = String::new();
   reader.read_line(&mut input).unwrap();
   let l: i64= input.trim().parse().unwrap();//문자열 길이
   let m: u64= 1234567891;//서로소
   let mut hash=0;
   input.clear();
   reader.read_line(&mut input).unwrap();
   let mut a:char;
   let mut r= 1;
   let  s= input.trim();
   for c in s.chars(){
       a = c;
      hash= (hash + (a as u64 - 96)*r)%m; //(a*r) mod M
      //R의 값은 26보다 큰 소수인 31
      r = (r * 31) % m; //pow(r,n) 값이 커지니 mod M나눠주기

   }
   writeln!(writer,"{}",hash).unwrap();
   writer.flush().unwrap();
}

