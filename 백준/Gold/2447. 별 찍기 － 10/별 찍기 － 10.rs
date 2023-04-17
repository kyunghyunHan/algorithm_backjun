use std::io::{stdin, BufRead, BufReader};

fn star(i :usize,j:usize,num:usize){
 
    if (i/num) %3==1 && (j/num)%3==1{
       print!(" ");
    }else{
        if num/3==0{
            print!("*");
        }else{
            star(i,j,num/3);
        }
    }
}
fn main() {
     let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut arr: Vec<usize>=vec! [0;41];
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap(); // 수정된 부분
    let mut nums = buffer.trim().split_whitespace();
    let n = nums.next().unwrap().parse::<usize>().unwrap();

   for i in 0..n{
    for j in 0..n{
        star(i,j,n);
    }
    print!("\n");
   }
}
