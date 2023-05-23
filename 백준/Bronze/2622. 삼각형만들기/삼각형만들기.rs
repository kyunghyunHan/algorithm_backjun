use std::io::{BufReader,BufWriter,BufRead,stdin,stdout};

fn main(){
   let mut reader=BufReader::new(stdin().lock());

   let mut input= String::new();

   reader.read_line(&mut input).unwrap();
let mut ans= 0;
let n= input.trim().parse::<isize>().unwrap();
for i in 1..n{
   for j in i..n{
      let k = n - i - j;

      if j>k{
         break;
      }else if i+j>k{
ans+=1;
      }
   }
}
println!("{}",ans);
}
