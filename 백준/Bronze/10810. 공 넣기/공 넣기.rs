use std::io;
fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let number_a: Vec<i32> = input_a
    .split_whitespace()
    .map(|x| -> i32 { x.parse().unwrap() })
    .collect();

 
    let n= number_a[0];
    let m= number_a[1];
    let mut  v:Vec<i32>= vec![0;n as usize];
    for s in 1..=m{
      let mut input_b = String::new();
      io::stdin().read_line(&mut input_b).unwrap();
      let number_b: Vec<i32> = input_b
      .split_whitespace()
      .map(|x| -> i32 { x.parse().unwrap() })
      .collect();
    let i = number_b[0];
    let j = number_b[1];
    let k= number_b[2];
      
       for u in i..=j{
           v[u as usize -1]=k;
       }

       
    }
   for i in 0..v.len(){
    print!("{:?} ",v[i as usize]);
   }
}