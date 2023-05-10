use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};


fn main(){

    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<isize>().unwrap());
    let k= nums.next().unwrap();
    let n= nums.next().unwrap();
let mut writer= BufWriter::new(stdout().lock());
let mut result=0;
   if n==1{
    result=-1;
   }else{
  result = (n*k)/ ( n-1);
    if (n*k)%(n-1)!=0{
        result+=1;
    }

   }
   writeln!(writer,"{}",result).unwrap();
}