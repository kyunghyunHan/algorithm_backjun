use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Stdout, Write};
use std::collections::HashMap;
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse::<i32>().unwrap();
    let mut hm= HashMap::new();
    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let d:Vec<String>= input.trim().split_whitespace().map(|x|x.to_string()).collect();
        hm.insert(d[0].clone(), d[1].parse::<i32>().unwrap());
    }
    let get= hm.get("jinju").unwrap();
    let mut count =0;
    for i in hm.iter(){
         if i.1 >get{
            count+=1;
         }
    }
    writeln!(writer,"{}",get).unwrap();
    writeln!(writer,"{}",count).unwrap();
    writer.flush().unwrap();
}