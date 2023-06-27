use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut writet= BufWriter::new(stdout().lock());
    let mut v:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
     

    
       /*무스를 구하기
   짝수 뾰족한 무스의 경우 Even x
   홀수 뾰족한 무스의 경우 Odd x
   아무것도 아닌경우 Not a moose
    */

    if v[0]==0 && v[1]==0{
        writeln!(writet,"{}","Not a moose").unwrap();

    }else if v[0]!=v[1]{
 
   writeln!(writet,"Odd {}",  usize::max(v[0], v[1])*2).unwrap();

    }else if v[0]==v[1]{
        writeln!(writet,"Even {}",v[0]+v[1]).unwrap();

    }


   
}
