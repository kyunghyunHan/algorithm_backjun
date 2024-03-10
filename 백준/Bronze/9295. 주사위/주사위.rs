use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input= String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse::<i32>().unwrap();
    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let ab= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
        let a= ab[0];
        let b= ab[1];
        writeln!(writer,"Case {}: {}",i+1,a+b).unwrap();

    }
    writer.flush().unwrap();

}