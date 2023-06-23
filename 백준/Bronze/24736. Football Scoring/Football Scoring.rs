use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();


    reader.read_line(&mut input).unwrap();

  let mut score= 0;

   let mut nums= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<usize>>();
   
  score= nums[0]*6+nums[1]*3+nums[2]*2+nums[3]*1+nums[4]*2;


   input.clear();
   reader.read_line(&mut input).unwrap();

   let mut home_score= 0;
   let mut nums2= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<usize>>();

   home_score= nums2[0]*6+nums2[1]*3+nums2[2]*2+nums2[3]*1+nums2[4]*2;

   writeln!(writer,"{} {}",score, home_score).unwrap();

}