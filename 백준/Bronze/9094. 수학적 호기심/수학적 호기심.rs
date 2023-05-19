use core::num;
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){



let mut reader= BufReader::new(stdin().lock());

let mut input = String::new();
reader.read_line(&mut input).unwrap();
let t= input.trim().parse::<usize>().unwrap();

for i in 0..t{
     input.clear();

     reader.read_line(&mut input).unwrap();
     let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
    let mut res= 0;
     let n= nums.next().unwrap();
     let m= nums.next().unwrap();

   for i in 1..n{
     for j in i+1..n{
        if (usize::pow(i, 2) +usize::pow(j, 2)+m)% (i*j)==0{
            res+=1;
        }
     }
   }

println!("{}",res)


}

}

