use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();
reader.read_line(&mut input).unwrap();
let mut n:i32= input.trim().parse().unwrap();


input.clear();
reader.read_line(&mut input).unwrap();
let mut n2:i32= input.trim().parse().unwrap();

let spped= n2-n;

if spped <=0 {
    writeln!(writer,"Congratulations, you are within the speed limit!").unwrap();
}else {
    if 1<spped && spped<=20 {
        writeln!(writer,"You are speeding and your fine is $100.").unwrap();

    }else if 21 <spped && spped <=30 {
        writeln!(writer,"You are speeding and your fine is $270.").unwrap();

    }else {
        writeln!(writer,"You are speeding and your fine is $500.").unwrap();

    }
}
}
