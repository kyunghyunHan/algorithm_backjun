use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};


fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut input =String::new();
    reader.read_line(&mut input).unwrap();
   let mut writer= BufWriter::new(stdout().lock());

    let n: usize =input.trim().parse().unwrap();

  
      let mut result= 1;
      let mut ans= 1;

    for i in 1..=n{
result*=i;
result%=10;
    }
    writeln!(writer,"{}",result).unwrap();
}

