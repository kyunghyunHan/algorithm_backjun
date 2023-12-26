use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    
     for _ in 0..15 {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let s= input.trim();
        if s.contains('w') {
            writeln!(writer,"chunbae").unwrap();
            break ;
        }else if  s.contains('b') {
            writeln!(writer,"nabi").unwrap();
            break ;
        }else if s.contains('g') {
            writeln!(writer,"yeongcheol").unwrap();
            break ;
        }
   
    }
   writer.flush().unwrap();
}
