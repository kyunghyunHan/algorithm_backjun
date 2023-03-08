use std::io;
fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: Vec<usize> = input
    .split_whitespace()
    .map(|x| -> usize { x.parse().unwrap() })
    .collect();
    

    for i in 1..=n[0]{
        let mut input_a= String::new();
        io::stdin().read_line(&mut input_a).unwrap();
        input_a.trim();
        let first = input_a.trim().chars().nth(0).unwrap();
        let last_char = input_a.trim().chars().last().unwrap();
        println!("{}{}",first,last_char);
    }
   
  
}