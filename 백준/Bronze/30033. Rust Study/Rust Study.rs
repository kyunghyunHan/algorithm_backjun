use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let  a:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let  b:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

    let mut count=  0;

    for i in 0..n{
        if b[i as usize]>=a[i  as usize]{
           count+=1;
        }
    }
    writeln!(writer,"{}",count).unwrap();
    writer.flush().unwrap();

}