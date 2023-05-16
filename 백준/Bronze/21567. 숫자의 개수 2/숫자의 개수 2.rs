use std::io::{BufWriter,BufReader,BufRead,Write,stdin,stdout};
fn main(){
let mut v=vec![0,0,0,0,0,0,0,0,0,0];
let mut reader= BufReader::new(stdin().lock());
 let mut result= 1;
 let mut writer= BufWriter::new(stdout().lock());
for i in 0..3{
let mut input= String::new();
reader.read_line(&mut input).unwrap();
let n =input.trim().parse::<usize>().unwrap();
  result *= n;
}
let mut result_s= result.to_string();

for i in 0..=v.len(){

    for j in 0..result_s.len(){
               if i.to_string()==result_s[j..j+1]{
                v[i]+=1;
               }
        
    }
}

for i in v{
    writeln!(writer,"{}",i).unwrap();
}

}