use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let input_result= input.trim();
  let mut writer= BufWriter::new(stdout().lock());
writeln!(writer,"{}",":fan::fan::fan:").unwrap();
writeln!(writer,":fan::{}::fan:",input_result).unwrap();
writeln!(writer,"{}",":fan::fan::fan:").unwrap();

}