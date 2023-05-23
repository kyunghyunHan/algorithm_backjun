use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){

   let mut reader= BufReader::new(stdin().lock());
   let mut input = String::new();
   reader.read_line(&mut input).unwrap();
   
   let  t= input.trim().parse::<usize>().unwrap();
   let mut writer= BufWriter::new(stdout().lock());
   for i in 0..t{
      input.clear();
      reader.read_line(&mut input).unwrap();
      let mut nums= input.trim().split_whitespace().map(|x|x.parse::<f32>().unwrap());

      let a= nums.next().unwrap();
      let b= nums.next().unwrap();


       writeln!(writer,"{}",(a*a/2.0)/(b*b/2.0)).unwrap();
   }

}