use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());

    

let mut writer= BufWriter::new(stdout().lock());
let mut input = String::new();
reader.read_line(&mut input).unwrap();

let n:usize= input.trim().parse().unwrap();

let mut ans= n%8;
if ans==1{
    writeln!(writer,"{}",1).unwrap();

}else if ans==2||ans==0{
    writeln!(writer,"{}",2).unwrap();
}else if ans ==3||ans==7{
    writeln!(writer,"{}",3).unwrap();

}else  if ans==4||ans==6{
    writeln!(writer,"{}",4).unwrap();

}else {
    writeln!(writer,"{}",5).unwrap();

}
}   