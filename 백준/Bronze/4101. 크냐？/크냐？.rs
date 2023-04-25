use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};

fn main(){
  let mut reader= BufReader::new(stdin().lock());


  let mut writer= BufWriter::new(stdout().lock());


  

  loop{
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
    let a= nums.next().unwrap();
    let b= nums.next().unwrap();
if a==0&&b==0{
    break;
}
if a>b{
      writeln!(writer,"Yes").unwrap();
}else{
    
    writeln!(writer,"No").unwrap();
}
   
  }




 
  

}