use std::io::{BufReader,BufRead,BufWriter,stdin,Write,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut v:Vec<i32>= Vec::new();
     let n= input.trim().parse().unwrap();
     for _ in 0..n{
         input.clear();
         reader.read_line(&mut input).unwrap();
         let t: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
             v.push(t.iter().skip(1).sum())
     }
     v.sort();
     let mut answer = 0;

     for i in 0..n {
         answer += (n - i) as i32 * v[i];

     }

     writeln!(writer,"{}",answer).unwrap();
     writer.flush().unwrap();
}