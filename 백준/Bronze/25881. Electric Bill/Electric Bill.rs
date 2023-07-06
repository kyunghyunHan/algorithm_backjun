use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut writer= BufWriter::new(stdout().lock());
    let mut nums:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

    input.clear();
    reader.read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s: i32 = input.trim().parse().unwrap();
        let mut fee= 0;
        if s>=1000{
            fee= 1000 * nums[0] + (s-1000)* nums[1];
        }else{
            fee= s * nums[0];
        }
        writeln!(writer,"{} {}",s,fee).unwrap();
    }
}