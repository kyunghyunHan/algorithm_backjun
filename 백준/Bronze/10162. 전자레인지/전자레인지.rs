use std::io::{BufReader,BufRead,BufWriter,stdin,stdout,Write};

fn main(){
let mut reader= BufReader::new(stdin().lock());

let mut input = String::new();
reader.read_line(&mut input).unwrap();
let mut writer= BufWriter::new(stdout().lock());
let n= input.trim().parse::<usize>().unwrap();
//상수할당
const a:usize = 60*5;
const b:usize = 60;
const c:usize = 10;

let mut t:Vec<usize>= vec![a,b,c];
let mut v:Vec<usize>= vec![0,0,0];
let mut result= n;
if (n%10 !=0){
writeln!(writer,"{}","-1").unwrap();

}else{
   for i in 0..3{
      v[i]= result/t[i];
      result%= t[i];

   }
   writeln!(writer,"{} {} {}",v[0],v[1],v[2]).unwrap();
}


}

