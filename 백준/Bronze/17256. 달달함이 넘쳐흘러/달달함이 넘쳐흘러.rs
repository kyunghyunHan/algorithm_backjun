use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
  let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

let ax= nums[0];
let ay= nums[1];
let az= nums[2];



  input.clear();
  
  reader.read_line(&mut input).unwrap();
  let mut nums2:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
  let cx= nums2[0];
  let cy= nums2[1];
  let cz= nums2[2];

  writeln!(writer,"{} {} {}",cx-az,cy/ay,cz-ax).unwrap();
}