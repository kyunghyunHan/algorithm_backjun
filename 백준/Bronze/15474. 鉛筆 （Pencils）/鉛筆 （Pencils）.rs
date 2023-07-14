use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();
reader.read_line(&mut input).unwrap();
let mut nums:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

let mut x_temp= 0;
let mut y_temp= 0;

let n= nums[0];
let x= nums[1];
let a= nums[2];
let y= nums[3];
let b= nums[4];

for i in 1..=n {
    if x*i >=n {
        x_temp= a*i;
        break;;
    }
}
for i in 1..=n {
    if y *i>=n{
        y_temp= b*i;
        break;
    }
}
writeln!(writer,"{}",i32::min(x_temp, y_temp)).unwrap();

}
