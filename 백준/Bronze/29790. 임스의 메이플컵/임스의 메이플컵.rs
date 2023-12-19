use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let  inputs:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let n= inputs[0];//문제해결개수
    let u= inputs[1];//유니온레벨
    let l= inputs[2];//최고레벨
    let mut sign =0;

    if n>=1000{
        sign=1
    }else{
        writeln!(writer,"Bad").unwrap();

    }

    if sign==1 && (u>=8000||l>=260){
        sign+=1
    }
   if sign==1{
    writeln!(writer,"Good").unwrap();

   }else if sign ==2{
    writeln!(writer,"Very Good").unwrap();

   }
    writer.flush().unwrap();


}