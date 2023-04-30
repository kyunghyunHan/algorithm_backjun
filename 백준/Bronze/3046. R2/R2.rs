use std::{io::{BufRead,BufWriter,BufReader,Write,stdin,stdout}, usize};

fn main(){


    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<isize>().unwrap());

   let mut r1= nums.next().unwrap();

 
   let mut s= nums.next().unwrap();

    writeln!(writer,"{}",(s*2)-r1).unwrap();


}