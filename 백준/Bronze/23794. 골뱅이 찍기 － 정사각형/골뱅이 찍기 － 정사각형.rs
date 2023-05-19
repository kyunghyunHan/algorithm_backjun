use core::num;
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){



let mut reader= BufReader::new(stdin().lock());

let mut input = String::new();
reader.read_line(&mut input).unwrap();
let t= input.trim().parse::<usize>().unwrap();
for _ in 0..t+2 {
    print!("@");
  }
  println!();
for i in 0..t{
    
    for j in 0..t+2{

        if j==0||j==t+1{
          print!("@")
        }else{
            print!(" ")
        }
 
    }
    println!()
}
for _ in 0..t+2 {
    print!("@");
  }

}

