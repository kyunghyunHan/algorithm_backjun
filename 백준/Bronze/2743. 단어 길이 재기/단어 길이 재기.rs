use std::io;
fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
     let mut num = 0;
    for i in input.chars(){
            num=num+1;
    }
    println!("{}",num-1)
}