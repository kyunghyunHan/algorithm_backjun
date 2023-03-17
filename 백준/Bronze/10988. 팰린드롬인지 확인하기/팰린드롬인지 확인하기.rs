use std::io;
fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
let mut v :Vec<i32>= vec![];
   
  for i in 0..input_a.trim().len()/2{
    if input_a.trim().chars().nth(i).unwrap()==input_a.trim().chars().rev().nth(i).unwrap(){

      v.push(1);
    }else{

      v.push(0);
    }
}
 
 let mut test= 0;
  for i in &v{
    test  = test+i;
  }
  
  if  v.len() == test as usize {
    println!("{}",1)

  }else{
    println!("{}",0)
  }

}