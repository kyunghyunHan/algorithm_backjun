use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse().unwrap();
    for _ in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let inputs:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
        let m: i32= inputs[0];
        let n: i32= inputs[1];
        let x: i32= inputs[2];
        let y: i32= inputs[3];
        writeln!(writer,"{}",calculate(m,n,x,y)).unwrap();
    }
    writer.flush().unwrap();
}

fn calculate(m:i32,n:i32,x:i32,y:i32)->i32{
    let mut k= x;
    while k<=m*n{
        if (k-x) % m ==0 && (k-y) % n==0{
            return k;
        }else{
            k+=m;
           
        }
    }
    return -1
}