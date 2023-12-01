use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());

    let colors = vec!["black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "grey", "white"];
let mut a=0;
let mut b=0;
let mut c=0;
let mut input: String = String::new();
reader.read_line(&mut input).unwrap();
let s1= input.trim();
let mut input: String = String::new();
reader.read_line(&mut input).unwrap();
let s2= input.trim();
let mut input: String = String::new();
reader.read_line(&mut input).unwrap();
let s3= input.trim();
a = match colors.iter().position(|&x| x == s1) {
    Some(index) => index,
    None => {
        println!("Color not found");
        return;
    }
};

 b = match colors.iter().position(|&x| x == s2) {
    Some(index) => index,
    None => {
        println!("Color not found");
        return;
    }
};

 c = match colors.iter().position(|&x| x == s3) {
    Some(index) => index,
    None => {
        println!("Color not found");
        return;
    }
};
    let result = (a.to_string() + &b.to_string()).parse::<u64>().unwrap() * 10_u64.pow(c as u32);
    
    writeln!(writer,"{}", result).unwrap();
    writer.flush().unwrap();
}


