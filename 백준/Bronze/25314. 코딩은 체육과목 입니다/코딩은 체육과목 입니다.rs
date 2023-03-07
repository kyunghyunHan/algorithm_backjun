use std::io;
fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num :Vec<usize>= input.split_whitespace().map(|x|->usize{x.parse().unwrap()}).collect();
    for i in 0..=num[0]/4 {
       if i != num[0]/4 {
        print!("long ")
       }else{
        print!("int")
       }
    }
}