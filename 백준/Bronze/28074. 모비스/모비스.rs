use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());

let mut input = String::new();
let mobis: Vec<char> = vec!['M', 'O', 'B', 'I', 'S'];
reader.read_line(&mut input).unwrap();
let n:Vec<char>= input.trim().chars().collect();

let mut result = true;

for i in mobis {
    if !n.contains(&i) {
        result = false;
        break;
    }
}

if result {
    println!("YES");
} else {
    println!("NO");
}
}
