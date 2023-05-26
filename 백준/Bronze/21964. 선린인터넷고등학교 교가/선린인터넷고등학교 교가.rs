use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
let mut reader= BufReader::new(stdin().lock());

let mut input= String::new();
reader.read_line(&mut input).unwrap();

let n= input.trim().parse::<usize>().unwrap();

let mut input2= String::new();
reader.read_line(&mut input2).unwrap();

let s= input2.trim().chars().collect::<Vec<char>>();

for i in n-5..n{
    print!("{}",s[i])
}

}