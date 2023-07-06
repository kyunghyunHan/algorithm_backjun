use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut writer= BufWriter::new(stdout().lock());
    let k :f32= input.trim().parse().unwrap();
     

    let mut ans= 25.0 + k * 0.01;

    if ans <= 100 as f32 {
        writeln!(writer,"{}",100.00).unwrap();
    }else if ans >= 2000 as f32 {
        writeln!(writer,"{}",2000.00).unwrap();
    }else {
        writeln!(writer,"{:.2}",ans).unwrap();
    }
}