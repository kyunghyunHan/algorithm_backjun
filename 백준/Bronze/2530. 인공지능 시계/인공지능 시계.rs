use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader=BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer=BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let n= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    let  mut h: i32 = n[0];
    let  mut m: i32 = n[1];

    let mut s: i32 = n[2];
   input.clear();
   reader.read_line(&mut input).unwrap();
   let mut d:i32= input.trim().parse().unwrap();
    s+=d %60;
    d= d/60;
    if s>=60 {
      s-=60;
      m+=1;
    }

    m+=d%60;
    d=d/60;
    if m>=60{
      m-=60;
      h+=1;
    }

    h+=d%24;
    if h >=24{
      h-=24;
    }

    writeln!(writer,"{} {} {}",h,m,s).unwrap();


     

}