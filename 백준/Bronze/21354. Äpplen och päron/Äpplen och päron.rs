use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut input= String::new();
  reader.read_line(&mut input).unwrap();
  let mut writer= BufWriter::new(stdout().lock());
  let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
   

   /*
   사과는 개당 SEK 7
   배는 개달 SEK 13

    */
   //axel이 판매한 사과의수
   let axel= nums[0] *7;
   //petra가 판매한 배의 수
   let petra= nums[1] *13;

   if axel >petra{
    writeln!(writer,"{}","Axel").unwrap();

   }else if axel< petra{
    writeln!(writer,"{}","Petra").unwrap();

   }else if axel ==petra{
    writeln!(writer,"{}","lika").unwrap();

   }
}   