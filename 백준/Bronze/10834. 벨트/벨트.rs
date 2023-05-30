use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
    let mut d= 0;
    let mut cur= 1;
    let mut reader= BufReader::new(stdin().lock());

    let mut input = String::new();
let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
 
   let n:usize= input.trim().parse().unwrap();
   for i in 0..n{
         input.clear();
         reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    if nums[2]==1 {
    d= 1-d;
    }
    cur= (cur as f32*(nums[1] as f32/nums[0]as f32))as i32;


   }
    writeln!(writer,"{} {}",d,cur).unwrap();
}