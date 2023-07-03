use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());

  let mut input = String::new();
  reader.read_line(&mut input).unwrap();
   let n = input.trim().parse().unwrap();
   /*
   w*k공간에 사람을 최대한 많이 눕여야함
   한사람은 정확힉 2칸차지
   W*k /2
    */
   for  i in 0..n {
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut nums:Vec<usize> = input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    writeln!(writer,"{}",nums[0]*nums[1]/2).unwrap();
   }

  
   
}