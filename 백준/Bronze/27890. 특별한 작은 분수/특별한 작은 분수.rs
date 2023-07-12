use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();

reader.read_line(&mut input).unwrap();
let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
  
let mut x= nums[0];
let y= nums[1];
for i in 0..y{
    if x%2==0{
        x= ( (x/2)^ 6);
    }else {
        x=(2*x)^6

    }
}
writeln!(writer,"{}",x).unwrap()
;
}
