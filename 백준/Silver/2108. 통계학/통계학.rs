use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::collections::HashMap;

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input= String::new();
  reader.read_line(&mut input).unwrap();
  let mut v:Vec<i32>= Vec::new();
  let  n=  input.trim().parse().unwrap();

  for _ in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let m:i32 = input.trim().parse().unwrap();
   v.push(m);
  }
  if n==1{
    writeln!(writer,"{}",v[0]).unwrap();
    writeln!(writer,"{}",v[0]).unwrap();
    writeln!(writer,"{}",v[0]).unwrap();
    writeln!(writer,"{}",0).unwrap();
    writer.flush().unwrap();
   return;
  }

  v.sort();
  //평균
  let  a= f64::round(v.iter().sum::<i32>() as f64/n as f64 ) as i32;
  //중앙값 계산
  let b= v[n/2];
  let mut num_map: HashMap<i32, i32> = HashMap::new();
  for i in 0..v.len() {
      let count = num_map.entry(v[i]).or_insert(0);
      *count += 1;
  }

  let mut numv: Vec<(i32, i32)> = num_map.into_iter().collect();
  numv.sort_by(|a, b| {
      if a.1 == b.1 {
          a.0.cmp(&b.0)
      } else {
          b.1.cmp(&a.1)
      }
  });

  let c = if numv[0].1 == numv[1].1 { numv[1].0 } else { numv[0].0 };


  let d = v[n - 1] - v[0];


  writeln!(writer,"{}",a).unwrap();
  writeln!(writer,"{}",b).unwrap();
  writeln!(writer,"{}",c).unwrap();
  writeln!(writer,"{}",d).unwrap();
  writer.flush().unwrap();
}