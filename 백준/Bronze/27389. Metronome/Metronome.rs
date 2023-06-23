use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();


    reader.read_line(&mut input).unwrap();

    let n: f64 = input.trim().parse().expect("Invalid input.");

    let revolutions = n / 4.0;

    println!("{:.2}", revolutions);

}