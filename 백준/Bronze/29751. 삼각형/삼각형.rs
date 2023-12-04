use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    

    writeln!(writer,"{:.1}",(n[0]*n[1]) as f32 *0.5).unwrap();

}