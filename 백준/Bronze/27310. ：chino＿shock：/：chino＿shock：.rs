use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();

reader.read_line(&mut input).unwrap();
let s:Vec<char>= input.trim().chars().collect();
let lens= s.len();
let mut colon =0;
let mut under = 0;
for i in 0..s.len(){
    if s[i]==':'{
        colon+=1;
    }else if s[i]=='_'{
        under+=1;
    }
}
writeln!(writer,"{}",lens+colon+under*5).unwrap();
}
