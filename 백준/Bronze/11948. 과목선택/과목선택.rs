use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};


fn main() {
    let mut reader= BufReader::new(stdin().lock());
   
   let mut writer= BufWriter::new(stdout().lock());
   let mut v: Vec<usize> = vec![];
   let mut v2: Vec<usize>= vec![];
for i in 0..6{
  
  let mut input =String::new();
  reader.read_line(&mut input).unwrap();
  let  n= input.trim().parse().unwrap();

 if i<4{
  v.push(n);
 }else {
  v2.push(n);

 }

  
}
v.sort();
v2.sort();
let mut result = 0;

for i in 1..4{
  result+=v[i];
}

result+=v2[1];
writeln!(writer,"{}",result).unwrap();
}

