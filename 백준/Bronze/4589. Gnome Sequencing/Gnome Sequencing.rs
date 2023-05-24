use std::io::{BufWriter,BufRead,BufReader,stdin,stdout};

fn main(){


   let mut reader= BufReader::new(stdin().lock());

   let mut input = String::new();

   reader.read_line(&mut input).unwrap();

   let n: usize= input.trim().parse::<usize>().unwrap();

   println!("Gnomes:");
   for i in 0..n{
      input.clear();
      reader.read_line(&mut input).unwrap();
      let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
      let a= nums.next().unwrap();
      let b= nums.next().unwrap();
      let c= nums.next().unwrap();
      if a > b &&b> c ||c>b && b>a{
         println!("Ordered")

      }else{
         println!("Unordered")
      }
   } 

}