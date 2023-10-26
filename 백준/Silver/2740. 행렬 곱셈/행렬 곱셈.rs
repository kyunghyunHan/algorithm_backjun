use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut pro1 = [[0; 101]; 101];
    let mut pro2 = [[0; 101]; 101];
    let mut res = [[0; 101]; 101];
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let mut v: Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let  n= v[0];//3
    let  m= v[1];//2


    for i in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut numbers = input.split_whitespace();
        for j in 0..m {
            pro1[i as usize][j as usize] = numbers.next().unwrap().parse().expect("Invalid input");
        }
    }
  
    

    input.clear();
    reader.read_line(&mut input).unwrap();
    v= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let  m2= v[0];
    let  k= v[1];

    for i in 0..m2 {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut numbers = input.split_whitespace();
        for j in 0..k {
            pro2[i as usize][j as usize] = numbers.next().unwrap().parse().expect("Invalid input");
        }
    }
  

    
    for i in 0..n {
        for j in 0..k {
            for x in 0..m {
                res[i as usize][j as usize] += pro1[i as usize][x as usize] * pro2[x as usize][j as usize];
            }
            print!("{} ", res[i as usize][j as usize]);
        }
        println!();
    }
     writer.flush().unwrap();
    
}

