use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main() {
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input = String::new();

   reader.read_line(&mut input).unwrap();
   let mut values = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
   let l = values.next().unwrap();
   let p = values.next().unwrap();   
    input.clear();

    reader.read_line(&mut input).unwrap();
    let  participants = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
    for num in participants {
      write!(writer,"{} ", num -l *p).unwrap();
  }
    writer.flush().unwrap();
}
