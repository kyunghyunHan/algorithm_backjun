use std::io::{BufWriter,BufReader,BufRead,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());

let mut input = String::new();
reader.read_line(&mut input).unwrap();

let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
let mut writer= BufWriter::new(stdout().lock());
let  n= nums.next().unwrap();
let t= nums.next().unwrap();


input.clear();
reader.read_line(&mut input).unwrap();

let mut nums2= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
let mut result= 0;
let mut count= 0;
for i in 0..n{
    let s= nums2.next().unwrap();

    if result + s<=t{
        result+=s;
        count+=1;
    }else{
        break;
    }
}


writeln!(writer,"{}",count).unwrap();
}