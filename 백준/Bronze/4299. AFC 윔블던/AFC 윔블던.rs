use std::io::{BufReader,BufWriter,BufRead,stdin,stdout};

fn main(){


    let mut reader= BufReader::new(stdin().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();

    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
    let a= nums.next().unwrap();
    let b= nums.next().unwrap();

     let mut x= 0;
     let mut y= 0;

     if a<b{
        println!("{}",-1);
     }else{
   x =(a+b)/2;
 y= (a-b)/2;
 if x+y==a && x-y==b{

    println!("{} {}",x ,y);
 }else{
    println!("{}",-1);
 }
     }
}