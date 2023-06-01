use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
let mut writer= BufWriter::new(stdout().lock());
    let n= input.trim().parse().unwrap();
    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();

        let n2:i32= input.trim().parse().unwrap();
        writeln!(writer,"{} {}",n2,n2).unwrap();
    }
}