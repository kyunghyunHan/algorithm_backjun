use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();


    reader.read_line(&mut input).unwrap();

   let s= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<usize>>();

   //지하철까지 걸어가는 시간
   let n= s[0];
   //a분뒤에 버스도착
   let a =s[1];
   // b분뒤에 지하철도착
   let b= s[2];

   if a<n+b-n{
    writeln!(writer,"{}","Bus").unwrap();
   }else if a==n+b-n{
    writeln!(writer,"{}","Anything").unwrap();

   }else if  a>n+b-n{
    writeln!(writer,"{}","Subway").unwrap();

   }

}