use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());

let mut input = String::new();
reader.read_line(&mut input).unwrap();
let r:u32= input.trim().parse().unwrap();
input.clear();
reader.read_line(&mut input).unwrap();
let c:u32= input.trim().parse().unwrap();

for i in 0..r{
    for j in 0..c{
        write!(writer,"*");
    }
    writeln!(writer).unwrap();
}
}
