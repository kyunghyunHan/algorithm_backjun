use std::io;
fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let n: Vec<i32> = num
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
   
    
    let mut number= 0;
    for i in input.chars(){
        number+=1;
        if number ==n[0]{
            println!("{}",i)
        } 
    }
}