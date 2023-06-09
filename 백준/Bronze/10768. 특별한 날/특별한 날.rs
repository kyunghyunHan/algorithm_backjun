use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};
use std::cmp;
fn main() {
 let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());

let mut input= String::new();



reader.read_line(&mut input).unwrap();

let n:usize=  input.trim().parse().unwrap();
input.clear();
reader.read_line(&mut input).unwrap();
let n2:usize=  input.trim().parse().unwrap();


if n==2 {
    if n2== 18{
        writeln!(writer,"Special").unwrap();

    }else if n2>18{
        writeln!(writer,"After").unwrap();

    }else if n2<18{
        writeln!(writer,"Before").unwrap();

    }
}else if n>2{
    writeln!(writer,"After").unwrap();

}else if n<2{
    writeln!(writer,"Before").unwrap();

}
}
