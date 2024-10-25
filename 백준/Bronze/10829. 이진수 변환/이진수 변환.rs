use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    match input.next(){
        Some(Ok(line))=>{
            let n :u128= line.trim().parse().unwrap();
            
            writeln!(writer,"{}",format!("{:b}",{n})).unwrap();

        }
        _=>{
            writeln!(writer,"{}","입력 ㄴ").unwrap();
        }
    }
    writer.flush().unwrap();
}