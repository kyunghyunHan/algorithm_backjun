use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut res = 0;

for i in 0..4 {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let path = iter.next().unwrap();
    let height: i32 = iter.next().unwrap().parse().expect("Invalid input");
    res += height * if path == "Stair" { 17 } else { 21 };

}
writeln!(writer,"{}",res).unwrap();
}
