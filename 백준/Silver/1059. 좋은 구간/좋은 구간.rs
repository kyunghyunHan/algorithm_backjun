use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
//최대 랜선수

fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input= String::new();
  reader.read_line(&mut input).unwrap();
 let l:u32= input.trim().parse().unwrap();
 input.clear();
 reader.read_line(&mut input).unwrap();
 let mut v= input.trim().split_whitespace().map(|x|x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
 input.clear();
 reader.read_line(&mut input).unwrap();
 let n:u32= input.trim().parse().unwrap();
 v.sort();
  
 if v.contains(&n) {
  println!("0");
} else{
  let mut min = 0u32;
  let mut max= 0u32;
   for num in v{
    if num<n{
      min = num;
    }else if num>n && max==0{
      max=num;
    }
   }
  max-=1;
  min +=1;
  writeln!(writer,"{}",(n-min)*(max-n+1)+(max-n)).unwrap(); 
}
writer.flush().unwrap();
}