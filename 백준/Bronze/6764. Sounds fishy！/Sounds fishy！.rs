use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
   
    let mut writet= BufWriter::new(stdout().lock());
   
    reader.read_line(&mut input).unwrap();  
    let mut a:u64= input.trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();  
    let mut b:u64= input.trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();  
    let mut c:u64= input.trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();  
    let mut d:u64= input.trim().parse().unwrap();


    /*
    증가하는 시퀸스:Fist Rising 예:3479
    엄격하게 감소하는 시퀸스: Fish Diving
    판독값이 동일하면 Fish At Constant Depth
    모두다르면 No Fish
     */
   //증가하는 시퀸스
    if a<b &&b<c&&c<d{
      writeln!(writet,"{}","Fish Rising").unwrap();
      //감소하는시퀸스
    }else if a>b &&b>c&&c>d{
      writeln!(writet,"{}","Fish Diving").unwrap();    
      //모두 동일

    }else if a==b&&b==c&&c==d{
      writeln!(writet,"{}","Fish At Constant Depth").unwrap();    

    }else {
      writeln!(writet,"{}","No Fish").unwrap();    

    }

}
