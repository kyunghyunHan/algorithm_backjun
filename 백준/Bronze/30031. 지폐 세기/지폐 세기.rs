use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut  input= String::new();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    let mut sum = 0;
    for _ in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let ab= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
        let a= ab[0];
        let b= ab[1];
        if a== 136{
            sum+=1000;
        }else if a==142{
            sum+=5000;
        }else if a==148{
            sum+=10000
        }else if a ==154{
            sum+=50000
        }
    }
    writeln!(writer,"{}",sum).unwrap();
    writer.flush().unwrap();
}