use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let v :Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

   

    let mut y= 0;
     let mut m = 0;

     for i in v{
         y+=(i/30+1)*10;
         m+=(i/60+1)*15;
         
     }

    if y>m{
        writeln!(writer,"M {}",m).unwrap();
    }else if y==m{
        writeln!(writer,"Y M {}",m).unwrap();

    }else{
        writeln!(writer,"Y {}",y).unwrap();

    }
    
  
}