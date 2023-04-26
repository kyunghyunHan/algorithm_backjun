use std::io::{BufWriter, BufRead, BufReader, Write, stdin, stdout};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    // Read n from input
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut num = input.trim().split_whitespace().map(|x| x.parse::<isize>().unwrap());
    let a: isize= num.next().unwrap();
     for i in 0..a{
      input.clear();
      reader.read_line(&mut input).unwrap();
      let result = input.trim();
     if result.len() >= 6&& result.len() <= 9 {
writeln!(writer,"{}","yes").unwrap();
     }else{
        writeln!(writer,"{}","no").unwrap();
     }

      

     }


}
