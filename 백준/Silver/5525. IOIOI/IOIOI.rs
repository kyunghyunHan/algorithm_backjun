use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let m:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let s= input.trim();

    let  (mut answer,mut i,mut count)= (0,0,0);

 
    while i < (m - 2) {
        if i + 2 < s.len() as i32 && &s[i as usize..i as usize+3] == "IOI" {

            i += 2;
            count += 1;
            if count == n {
                answer += 1;
                count -= 1;
            }
        } else {
            i += 1;
            count = 0;
        }
    }
    
  
    writeln!(writer,"{}",answer).unwrap();
    writer.flush().unwrap();
}