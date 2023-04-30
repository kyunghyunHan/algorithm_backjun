use std::{io::{BufRead,BufWriter,BufReader,Write,stdin,stdout}, usize};

fn main(){


    let mut reader= BufReader::new(stdin().lock());
  
    let mut writer= BufWriter::new(stdout().lock());
for i in 0..3{
  let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<isize>().unwrap());

   let  r1= nums.next().unwrap();
   let  r2= nums.next().unwrap();
   let r3= nums.next().unwrap();
   let  r4= nums.next().unwrap();
  let  result= r1+r2+r3+r4;
   if result==4{
    writeln!(writer,"{}","E").unwrap();
   }else if result==3{
    writeln!(writer,"{}","A").unwrap();

   }else if result==2{
    writeln!(writer,"{}","B").unwrap();

   }else if result==1{
    writeln!(writer,"{}","C").unwrap();

   }else if result==0{
    writeln!(writer,"{}","D").unwrap();

   }
}
  
    

    


}