use std::io::{BufWriter,BufReader,BufRead,Write,stdin,stdout};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());

 /*
  x층에 놓기로햇다면
  x*사람수 *2
 

10
20
30
인 경우 각 층에 커피머신을 둘 경우 계산을 해봤을 때
0층에 두기 : 10*2 + 30*4 = 596
1층에 두기 : 20*2 + 30*2 = 398
2층에 두기 : 10*2 + 20*4 = 592
  */
  let mut a_v:Vec<usize>= vec![];
  for i in 0..3{
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let a:usize= input.trim().parse().unwrap();
   a_v.push(a)
  }
  let mut r_v:Vec<usize>= vec![ a_v[1]*2 +a_v[2]*4,a_v[0]*2 +a_v[2]*2,a_v[0]*4 +a_v[1]*2];
  
  r_v.sort();
  writeln!(writer,"{}",r_v[0]).unwrap();
} 