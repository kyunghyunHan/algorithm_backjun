use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
   let mut reader= BufReader::new(stdin().lock());
   let mut writer= BufWriter::new(stdout().lock());
   let mut input = String::new();

   reader.read_line(&mut input).unwrap();
   let n:i32= input.trim().parse().unwrap();
   input.clear();
   reader.read_line(&mut input).unwrap();

   let nums= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
   //맨처음에 0
   //딸기->초코
   //초코->바나나
   //바나나->딸기
   //0 은 딸기
   //

   let mut count=0;
   let mut milk: i32= 0;
   for i in nums{

      if i ==count %3{
         count+=1;
      }






      
   }
   writeln!(writer,"{}",count).unwrap();
}