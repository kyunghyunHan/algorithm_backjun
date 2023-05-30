use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
  
    let mut input =String::new();
    reader.read_line(&mut input).unwrap();
   let s= input.trim();
   let mut  v:Vec<String>= vec![];

   for i in s.chars(){
     v.push(i.to_string())
   }
   
   let mut result=10;
   for i in 1..v.len(){

    if v[i]==v[i-1]{
        result+=5;
    }else{
        result+=10;
    }
   }
 
 println!("{}",result);
}