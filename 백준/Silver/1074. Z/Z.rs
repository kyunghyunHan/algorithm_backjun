use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    
    reader.read_line(&mut input).unwrap();
    let  inputs:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let  n= inputs[0];
    let  r= inputs[1];
    let  c= inputs[2];
    writeln!(writer,"{}",sol(n, r, c)).unwrap();
    writer.flush().unwrap();
}

fn sol(n:i32,r:i32,c:i32)->i32{
    if n==0{
        return 0;
    }else{
         return 2*(r% 2)+(c%2)+4*sol(n-1,(r as f32/2 as f32)as i32,(c as f32/2 as f32)as i32 )
    }

}