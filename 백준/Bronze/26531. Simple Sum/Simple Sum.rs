use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());

  let mut input = String::new();
  reader.read_line(&mut input).unwrap();
  

  let mut nums:Vec<&str>= input.trim().split_whitespace().collect();

  let a:i32= nums[0].parse().unwrap();

  let b:i32= nums[2].parse().unwrap();
  let c:i32= nums[4].parse().unwrap();

if a+b==c{
  writeln!(writer,"{}","YES").unwrap();
}else{
  writeln!(writer,"{}","NO").unwrap();

}
  
   
}