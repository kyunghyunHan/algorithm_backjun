use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};
use std::cmp;
fn main() {
 let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());

let mut v= [0;26];

let s_v= [
    'A', 'B', 'C', 'D', 'E', 'F',
    'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z'
  ];
let mut input = String::new();

reader.read_line(&mut input).unwrap();

let s =input.trim().chars().collect::<Vec<char>>();
for i in 0..s.len(){
for j in 0..s_v.len(){
    if s[i]==s_v[j].to_ascii_lowercase(){
    v[j]+=1;
    }
}

}
for i in v{
    write!(writer,"{} ",i).unwrap();
}
}
