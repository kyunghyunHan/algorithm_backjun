use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());

   for i in 0..3 {
    let mut input: String =String::new();
    reader.read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();

    let mut result:i128= 0;
    for i in 0..n{
        let mut input: String =String::new();
        reader.read_line(&mut input).unwrap();
        let n:i128 = input.trim().parse().unwrap();
        result+=n
    }
    if result ==0 {
        writeln!(writer,"{}",0).unwrap();
    }else if result > 0 {
        writeln!(writer,"{}","+").unwrap();

    }else if result < 0 {
        writeln!(writer,"{}","-").unwrap();
    }
   }
}