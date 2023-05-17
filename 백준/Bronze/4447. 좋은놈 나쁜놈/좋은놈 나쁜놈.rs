use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());

  let mut input= String::new();

  reader.read_line(&mut input).unwrap();


  let n= input.trim().parse::<usize>().unwrap();
let mut writer= BufWriter::new(stdout().lock());


  for i in 0..n{
     input.clear();
     reader.read_line(&mut input).unwrap();

     let n= input.trim();
     let mut g= 0;
     let mut b= 0;
     for i in n.chars(){
        if  i =='G' ||i=='g'{
            g+=1;
        }else if i=='B' || i=='b'{
            b+=1;
        }


     }

     if g>b{
        writeln!(writer,"{} is GOOD",n).unwrap();
     }else if g<b{
        writeln!(writer,"{} is A BADDY",n).unwrap();
     }else if g==b{
        writeln!(writer,"{} is NEUTRAL",n).unwrap();
     }





  }



}