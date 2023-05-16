use std::io::{BufWriter,BufReader,BufRead,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());


let mut input = String::new();
reader.read_line(&mut input).unwrap();
let mut writer= BufWriter::new(stdout().lock());
let n= input.trim().parse::<usize>().unwrap();
for i in 0..n{
    input.clear();

    reader.read_line(&mut input).unwrap();
    let mut cnt= 0;
    let s= input.trim().parse::<usize>().unwrap();
    for i in 0..101{
        if i*i+i<=s{
            cnt= i
        }
    }
   writeln!(writer,"{}",cnt).unwrap();

}
}