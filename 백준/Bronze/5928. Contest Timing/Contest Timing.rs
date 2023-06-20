use std::io::{BufReader,BufRead,BufWriter,stdin,stdout,Write};

fn main(){
    let mut reader=BufReader::new(stdin().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let mut writer= BufWriter::new(stdout().lock());
    let mut nums= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();

    let d= nums[0];
    let h =  nums[1];
    let m = nums[2];
    let pivot = 11 + 11 * 60 + 11 * 60 * 24;
    let ans = m + h * 60 + d * 60 * 24 - pivot;
    if ans < 0 {
        writeln!(writer,"-1").unwrap();
    } else {
        writeln!(writer,"{}", ans).unwrap();
    }
}