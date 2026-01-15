use std::io::{BufReader,Read,stdin};

fn main(){
    let mut input = String::new();
    BufReader::new(stdin().lock()).read_to_string(&mut input).unwrap();
    
    let mut it = input.split_whitespace();
    
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut cnt = 0;
    let mut arr = vec![];
    let a = (n-n/2) as usize;
    for _ in 0..n{
        let correct = it.next().unwrap().as_bytes()[0];
        arr.push(correct);
    }
    
     for i in 0..n{
        let correct = it.next().unwrap().as_bytes()[0];
        if arr[i] == correct{
            cnt+=1;
         }
    }
    
    
    
    println!("{cnt}");
}