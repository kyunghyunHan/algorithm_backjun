use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let mut writer= BufWriter::new(stdout().lock());
    let n= input.trim().parse().unwrap();
    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let ns:i32= input.trim().parse().unwrap();
        if ns%2==0{
            writeln!(writer,"{} is even",ns).unwrap();
        }else{
            writeln!(writer,"{} is odd",ns).unwrap();
        }
    }
}
