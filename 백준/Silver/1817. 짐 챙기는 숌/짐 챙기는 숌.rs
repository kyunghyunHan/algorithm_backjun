use std::{io::{BufReader,BufRead,BufWriter,Write,stdin,stdout}, process::exit};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let  nums= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<u64>>();

    let n= nums[0];
    let m=nums[1];
    if n==0{
      writeln!(writer,"{}",0).unwrap();
      writer.flush().unwrap();

      std::process::exit(0);
    }
    input.clear();
    reader.read_line(&mut input).unwrap();
    let  list= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<u64>>();
    let mut count= 1;
    let mut curr: u64= 0;
  
    for i in list{
       if i+curr>m{
        curr=i;
        count+=1;
       }else{
        curr+=i;
       }
       
    }

  writeln!(writer,"{}",count).unwrap();
  writer.flush().unwrap();
}