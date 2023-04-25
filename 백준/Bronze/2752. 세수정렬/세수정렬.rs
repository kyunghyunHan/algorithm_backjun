use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut input= String::new();

  reader.read_line(&mut input).unwrap();

  let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());

  let a= nums.next().unwrap();
  let b= nums.next().unwrap();
  let c= nums.next().unwrap();
  

  let mut v= [a,b,c];
  let mut writer= BufWriter::new(stdout().lock());
  v.sort();


  writeln!(writer,"{} {} {}",v[0],v[1],v[2]).unwrap();
  

}