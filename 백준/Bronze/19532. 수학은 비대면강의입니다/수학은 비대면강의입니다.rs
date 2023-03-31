use std::io;

fn main() {
  let  mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();


  let mut  v: Vec<i32> = input
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
  
  let mut a= v[0];
  let mut b= v[1];
  let mut c= v[2];
  let mut d= v[3];
  let mut e= v[4];
  let mut f= v[5];

  for i in -999..1000{

    for j in -999..1000{
        if a*i+b*j==c{
            if d*i +e *j==f{
                println!("{} {}",i,j)
            }
        }
    }
  }



}