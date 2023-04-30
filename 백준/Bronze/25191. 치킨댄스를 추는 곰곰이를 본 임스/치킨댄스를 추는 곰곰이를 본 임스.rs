use std::{io::{BufRead,BufWriter,BufReader,Write,stdin,stdout}, usize};

fn main(){


    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let mut chicken= input.trim().parse::<usize>().unwrap();
     let mut input2= String::new();
     reader.read_line(&mut input2).unwrap();

   let mut nums= input2.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
   let mut coke= nums.next().unwrap();
   let mut beer= nums.next().unwrap();


 

    writeln!(writer,"{}", std::cmp::min(chicken, coke / 2 + beer)).unwrap();


}