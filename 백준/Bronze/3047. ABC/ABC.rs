use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut input = String::new();
reader.read_line(&mut input).unwrap();
let mut writer= BufWriter::new(stdout().lock());
let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
let a= nums.next().unwrap();
let b= nums.next().unwrap();
let c= nums.next().unwrap();
let mut v:Vec<usize>=vec![a,b,c];
v.sort();

input.clear();
reader.read_line(&mut input).unwrap();
let result= input.trim();
for i in result.chars(){
    if i =='A'{
   write!(writer,"{} ",v[0]).unwrap();
    }else if i=='B'{
        write!(writer,"{} ",v[1]).unwrap();
    }
    else if i=='C'{
        write!(writer,"{} ",v[2]).unwrap();
    }
}



}


