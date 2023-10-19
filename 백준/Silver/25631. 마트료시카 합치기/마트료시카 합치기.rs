use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap(); 
    let mut  n:i32 = input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap(); 
    let  mut v:Vec<i32> = input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    v.sort();
    let mut left = 0;

    for i in 1..v.len(){
        if v[left as usize] < v[i] {
            left += 1;

            n-=1;
        }

    }
   writeln!(writer,"{}",n).unwrap();
    writer.flush().unwrap();
}