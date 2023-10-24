use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader =  BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

   let s= input.trim().chars().collect::<Vec<char>>();
   let mut zero = 0;
   let mut one = 0;

   if input.chars().nth(0) == Some('0') {
       zero += 1;
   } else {
       one += 1;
   }

   for i in 1..s.len() {
       if input.chars().nth(i) != input.chars().nth(i - 1) {
           if input.chars().nth(i) == Some('0') {
               zero += 1;
           } else {
               one += 1;
           }
       }
   }

   writeln!(writer,"{}",std::cmp::min(zero, one)).unwrap();
}
