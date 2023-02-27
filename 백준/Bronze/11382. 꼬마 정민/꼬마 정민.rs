use std::io;
fn main(){
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let number_arr: Vec<usize> = input_a
        .split_whitespace()
        .map(|x| -> usize { x.parse().unwrap() })
        .collect();
   
    println!("{}",number_arr[0]+number_arr[1]+number_arr[2])
 
}