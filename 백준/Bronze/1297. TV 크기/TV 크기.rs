use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let numsber= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();

    let len = numsber[0];
    let height = numsber[1];
    let width= numsber[2];
    let projection_height = ((len as f64) / ((height.pow(2) + width.pow(2)) as f64).sqrt() * (height as f64)) as i32;
    let projection_width = ((len as f64) / ((height.pow(2) + width.pow(2)) as f64).sqrt() * (width as f64)) as i32;
    
    writeln!(writer,"{} {}",projection_height, projection_width).unwrap();
}