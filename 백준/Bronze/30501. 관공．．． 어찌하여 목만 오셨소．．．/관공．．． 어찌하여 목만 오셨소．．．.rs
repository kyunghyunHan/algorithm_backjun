use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let n=input.trim().parse().unwrap();
    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s= input.trim();
        if s .contains("S"){
            writeln!(writer,"{}",s).unwrap();
            break;
        }
    }
}