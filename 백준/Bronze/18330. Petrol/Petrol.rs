use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut writet= BufWriter::new(stdout().lock());
    let mut o:u64= input.trim().parse().unwrap();     
    /*
    1인당 한달에 60리터의 연료카드
    휘발유 1리터당 1500 Oshloobs
    추가 주유 비용은 리터당 3000 Oshloobs
    
    
    
     */
   

   input.clear();
   reader.read_line(&mut input).unwrap();
   let mut o2:u64= input.trim().parse().unwrap();     
   let mut sale= 60+o2;


  
   if o<=sale{
    writeln!(writet,"{}",o*1500).unwrap();

   }else{
    writeln!(writet,"{}",sale*1500+(o-sale)*3000).unwrap();

   }
}
