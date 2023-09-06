use std::io::{BufRead,BufReader,Write,stdin,stdout,BufWriter};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
   

    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let num:i32= input.trim().parse().unwrap();
    

    let mut res= 1;
    for i in 11..num+1{
        res*=i;
    }
    writeln!(writer,"{}",6*res).unwrap();

    
}