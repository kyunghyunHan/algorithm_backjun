use std::io;

fn main() {
  let  mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();


  let mut  v1: Vec<i32> = input
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
 
 let mut n =v1[0];
 let mut k= v1[1];

    println!("{}",factorial(n)/(factorial(k)*factorial(n-k)));
}

fn factorial(n:i32)->i32{
    if n==1||n==0{
         return 1;
    }else{
        factorial(n-1)*n
    }
}