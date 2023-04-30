use std::{io::{stdin, stdout, BufRead, BufReader, BufWriter, Write}, usize};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    
    let mut input = String::new();
   reader.read_line(&mut input).unwrap();
   let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
   let  a= nums.next().unwrap();
   //스네이크 초기값
   let  mut b: usize= nums.next().unwrap();
    
    let mut input2= String::new();
    reader.read_line(&mut input2).unwrap();
  let mut nums2= input2.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
    //과일을 배열에 삽입
    let mut v:Vec<usize>= vec![];
    for i in 0..a{
    let num= nums2.next().unwrap();
      v.push(num);
    }
    //배열정렬
   v.sort();
   //정렬해서 가장 길이가 작은 과일하고 일치하면 스네이크 길이 +1
    for i in v{
      if b>=i {
        b+=1
      }
    }
    writeln!(writer, "{}", b).unwrap();
    writer.flush().unwrap();
}
