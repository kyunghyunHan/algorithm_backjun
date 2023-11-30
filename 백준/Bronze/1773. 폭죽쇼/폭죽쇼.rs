use std::{io ::{BufReader,BufWriter,BufRead,Write,stdin,stdout}, os::unix::process};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<i32>().unwrap());    
     let n= nums.next().unwrap();
     let c= nums.next().unwrap();

     let mut v = vec![0; c as usize + 1];

     for _ in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let m:i32= input.trim().parse().unwrap();
        if m==1{
            writeln!(writer,"{}",c).unwrap();
            writer.flush().unwrap();
            std::process::exit(0);
        }
        for k in (m..=c).step_by(m as usize) {
            v[k as usize] = 1;
        }
     }


     let sum: i32 = v.iter().sum();
     writeln!(writer,"{}",sum).unwrap();
    writer.flush().unwrap();
}