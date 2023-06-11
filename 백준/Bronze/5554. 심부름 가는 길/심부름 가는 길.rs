use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());

    

let mut writer= BufWriter::new(stdout().lock());
    let mut result= 0;
    for i in 0..4{
        let mut input =String::new();
        reader.read_line(&mut input).unwrap();
        let n:i32= input.trim().parse().unwrap();

        result+=n;
    }
  let x= result/60;
  let y= result%60;
    writeln!(writer,"{}\n{}",x,y).unwrap();

}