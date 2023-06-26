use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut n= input.trim().parse().unwrap();

     for i in 0..n{
      input.clear();
      reader.read_line(&mut input).unwrap();
      let mut nums :Vec<u128>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
      

      let l= nums[0];
      let w= nums[1];

      let le = nums[2];
      let we= nums[3];



     if l*w >le*we{
      writeln!(writer,"{}","TelecomParisTech").unwrap();
     }else if le*we>l*w{
      writeln!(writer,"{}","Eurecom").unwrap();

     }else{
            writeln!(writer,"{}","Tie").unwrap();

     }

     }


     
}