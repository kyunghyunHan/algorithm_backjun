use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};
use std::cmp;
fn main(){
  let mut reader= BufReader::new(stdin().lock());


  let mut writer= BufWriter::new(stdout().lock());
  let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
    let a= nums.next().unwrap();
    let b= nums.next().unwrap();
   let n1= cmp::min(a,b);
   let n2= cmp::max(a,b);

   let  mut n: usize= n2-n1-1;
if n1==n2 ||n1+1==n2{
    n=0
}
writeln!(writer,"{}",n).unwrap();

for i in n1+1..n2{
    write!(writer,"{} ",i).unwrap();
}

 
  

}