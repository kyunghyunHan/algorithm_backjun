use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
 
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut stack = 0;
    loop {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let s= input.trim().to_string();
        
        if s.len() >= 15 {
            if s=="고무오리 디버깅 시작"{
                continue;
            }else{
                    break;
            }

        
        }
        
        if s.starts_with("문제") {
            stack += 1;
        } else {
            if stack == 0 {
                stack += 2;
            } else {
                stack -= 1;
            }
        }
    }
    if stack== 0 {
        writeln!(writer,"고무오리야 사랑해").unwrap();
    } else {
        writeln!(writer,"힝구").unwrap();
    }
    writer.flush().unwrap();
}
