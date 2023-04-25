use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout());
    let mut lines = reader.lines();
    
    // 첫 번째 입력값 처리
    let a = lines.next().unwrap().unwrap().parse::<isize>().unwrap();
    
   if a==0{
       writeln!(writer, "{}", "YONSEI").unwrap();
   }else{
      writeln!(writer, "{}", "Leading the Way to the Future").unwrap();
   }
   
}