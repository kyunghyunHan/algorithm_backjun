use std::io;
fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let number_a: Vec<usize> = input_a
    .split_whitespace()
    .map(|x| -> usize { x.parse().unwrap() })
    .collect();
    let n= number_a[0];
    
    for i in 1..=n{

      for j in 0..n-i {
        print!(" ");
        
      }
      for k in 0..2*i-1{  
        print!("*");
      }
      print!("\n");
      
      

    }
    for i in 1..n{
      for j in 0..i{
      print!(" ");
      }
      for k in 0..2*n-(2*i+1){
      print!("*");
      }
       print!("\n");
    }
}