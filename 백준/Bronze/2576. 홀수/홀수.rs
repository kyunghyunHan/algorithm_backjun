use std::io::{BufRead, BufReader, BufWriter, stdin, stdout, Write};


fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

  
    let mut v:Vec<usize>= vec![];
    
   for i in 0..7{
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse::<usize>().unwrap();
    if n%2!=0{
    v.push(n)
    }

   }
   v.sort();
   let mut sums= 0;
   for i in &v{
    sums+=i
   }
   
  if v.len()!=0{
     writeln!(writer,"{}",sums).unwrap();
     writeln!(writer,"{}",v[0]).unwrap();
  }else{
    writeln!(writer,"{}",-1).unwrap();
  }
}
