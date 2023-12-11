use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut  input= String::new();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    let mut gifticon = 0;
    for _ in (0..n).into_iter(){
        input.clear();
        reader.read_line(&mut input).unwrap();
        let period= input.trim().replace("D-", "").parse::<i32>().unwrap();
        if period <=90{
            gifticon+=1;
        }

        
    }
    writeln!(writer,"{}",gifticon).unwrap();
    writer.flush().unwrap();
}