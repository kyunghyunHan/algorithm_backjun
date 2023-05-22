use std::io::{BufWriter,BufRead,BufReader,stdin,stdout};

fn main(){


    let mut reader= BufReader::new(stdin().lock());

     let mut input = String::new();
     reader.read_line(&mut input).unwrap();
    let n= input.trim().parse::<usize>().unwrap();

    let mut input2= String::new();

    reader.read_line(&mut input2).unwrap();

   let mut nums= input2.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
let mut result= 0;
let mut sum= 0;
   for i in 0..n{
    let s= nums.next().unwrap();

result= s*(i+1)-sum;
print!("{} ",result);
sum+=result;
   }


}