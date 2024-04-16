use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let m = input.trim().parse::<i32>().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let k = input.trim().parse::<i32>().unwrap();

    writeln!(writer,"{}",(m/n)*k).unwrap();
}