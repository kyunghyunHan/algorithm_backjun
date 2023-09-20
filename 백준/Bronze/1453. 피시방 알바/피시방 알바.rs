use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut nums = input.trim().split_ascii_whitespace().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
     nums.sort();
     let mut count = 0;
     let mut ans= 0;
     for i in nums{
        
         if i==ans {
            count+=1;
         }
         ans = i;
     }

     writeln!(writer,"{}",count).unwrap();
    writer.flush().unwrap();
}