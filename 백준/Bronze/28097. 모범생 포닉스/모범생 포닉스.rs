use std::{io::{BufReader,BufRead,BufWriter,Write,stdin,stdout}, iter::Sum};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut input= String::new();
let mut writer= BufWriter::new(stdout().lock());
reader.read_line(&mut input).unwrap();
let mut n:usize= input.trim().parse().unwrap();
/*
1.포닉스는 n개 의 공부 계획을 가지고 있다   
  2.i 공부 계획을 실행하는데 T시간이 필요
3.계획사이에는 8시간의 휴식이필요
 */
input.clear();
reader.read_line(&mut input).unwrap();
let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
let mut time= 0;
for i in nums{
  time+=i;
}
time= time+((n-1)*8);

let mut day =0;
let mut min = 0;
if time>=24{
  day=  time/24;
  min= time%24;
}else{
 min= time;
}

writeln!(writer,"{} {}",day,min).unwrap();

}
