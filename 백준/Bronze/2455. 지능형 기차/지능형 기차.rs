use std::io::{BufReader,BufRead,BufWriter,stdin,stdout,Write};

fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer: BufWriter<std::io::StdoutLock>= BufWriter::new(stdout().lock());

let mut max= 0;
let mut result = 0;
for i in 0..4{
   let mut input = String::new();
   reader.read_line(&mut input).unwrap();
  let mut nums= input.trim().split_whitespace().map(|x|x.parse::<isize>().unwrap()).collect::<Vec<isize>>();

  result+=(nums[1]-nums[0]);
  if result >max{
   max= result;
  }
}
writeln!(writer,"{}",max).unwrap();
}