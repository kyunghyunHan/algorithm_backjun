use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){


let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());





loop{
    let mut input= String::new();

    reader.read_line(&mut input).unwrap();
    let n:usize= input.trim().parse().unwrap();
    if n ==0{
        break;
    }
    writeln!(writer,"{}",n*(n+1)/2).unwrap();
}
}
