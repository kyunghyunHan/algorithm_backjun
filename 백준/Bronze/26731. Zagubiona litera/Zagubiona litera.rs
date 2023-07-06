use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut writer= BufWriter::new(stdout().lock());
    let mut v :Vec<char>= vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

    let s:Vec<char>= input.trim().chars().collect();

    for i in 0..v.len(){
            for j in &s{
              if v[i]==*j{
                 v[i]='0';
               }        
            }
    }
    for i in v{
        if i!='0'{
            writeln!(writer,"{}",i).unwrap();
        }
    }
}