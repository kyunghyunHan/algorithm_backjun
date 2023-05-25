use std::io::{BufReader,BufRead,BufWriter,stdin,stdout,Write};

fn main(){
let mut reader= BufReader::new(stdin().lock());

let mut input = String::new();
reader.read_line(&mut input).unwrap();
let mut writer= BufWriter::new(stdout().lock());
let n: usize= input.trim().parse::<usize>().unwrap();
let mut result = 0;
for i in 0..n{
   //
   let mut result= 0;
  
   let mut input = String::new();
   reader.read_line(&mut input).unwrap();
   let mut car_price: usize= input.trim().parse::<usize>().unwrap();
   result+=car_price;
   let mut input2 = String::new();
   reader.read_line(&mut input2).unwrap();
   let mut option_len: usize= input2.trim().parse::<usize>().unwrap();

   if option_len>0{
      for i in 0..option_len{
         let mut input = String::new();
         reader.read_line(&mut input).unwrap();
         let mut option_v= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
         result+=option_v[ 0]*option_v[1 ];

      }
   }
  
  writeln!(writer,"{}",result).unwrap();

}
}

