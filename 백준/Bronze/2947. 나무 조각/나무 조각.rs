
use std::{io::{BufWriter,BufReader,BufRead,Write,stdin,stdout}, result};

fn main(){

let mut reader= BufReader::new(stdin().lock());
let mut result:(usize,usize)= (0,0);
let mut writer= BufWriter::new(stdout().lock());



   let mut buffer= String::new();
reader.read_line(&mut buffer).unwrap();

let mut nums= buffer.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
let a= nums.next().unwrap();
let b= nums.next().unwrap();
let c= nums.next().unwrap();
let d= nums.next().unwrap();
let e= nums.next().unwrap();

let mut v:Vec<usize>= vec![a,b,c,d,e];

let mut i= 0;
loop{
  
   if  v[i] > v[i+1] {
      v.swap(i, i+1);
      println!("{} {} {} {} {}",v[0],v[1],v[2],v[3],v[4]);
   }
 
   if i==3 && v!=vec![1,2,3,4,5]{
     i=0
   } else if v== vec![1,2,3,4,5]{
      break;
   }
   else{
       i+=1;
   }
  

   
}



   


}