use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};


fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut input = String::new();
let mut writer= BufWriter::new(stdout().lock());
reader.read_line(&mut input).unwrap();
let mut ar= input.trim().split_whitespace().map(|x|x).collect::<Vec<&str>>();

let x= ar[0].parse::<usize>().unwrap();
let y=ar[2].parse::<usize>().unwrap();
let z= ar[4].parse::<usize>().unwrap();

if x+y==z{
    writeln!(writer,"YES").unwrap();
}else{
    writeln!(writer,"NO").unwrap();

}
}