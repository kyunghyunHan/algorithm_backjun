use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();


    reader.read_line(&mut input).unwrap();

   let n: i32= input.trim().parse().unwrap();
   for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let nums= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<f64>>();
    
    let d = nums[0];
    let f = nums[1];
    let p = nums[2];
    let cost = d * f * p;

writeln!(writer,"${:.2}",cost).unwrap();

   }
}