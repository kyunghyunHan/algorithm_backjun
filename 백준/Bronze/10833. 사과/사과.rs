use std::io::{BufWriter,BufRead,BufReader,stdin,stdout};
fn main(){


    let mut reader= BufReader::new(stdin().lock());

    let mut writer= BufWriter::new(stdout().lock());

    let mut input= String::new();

   reader.read_line(&mut input).unwrap();

   let n:usize= input.trim().parse().unwrap();
//결과값 출력
  let mut sum= 0;
   for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());

    let a= nums.next().unwrap();
    let d= nums.next().unwrap();


 if a<d{
    //나눈값 더하기
   sum+=d%a
 }else if a>d{
    //
    sum+=d;
 }
   }
   println!("{}",sum)
}