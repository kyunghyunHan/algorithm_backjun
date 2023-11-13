use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input = String::new();
  reader.read_line(&mut input).unwrap();
  //명령의 수
  let n:i64= input.trim().parse().unwrap();
  let mut s: Vec<i32> = vec![];
  for _ in 0..n {
      input.clear();
      reader.read_line(&mut input).unwrap(); // 수정된 부분
      let mut nums = input.trim().split_whitespace();
      let n = nums.next().unwrap();

      // 1. push
      if n == "push" {
          let n2 = nums.next().unwrap().parse::<i32>().unwrap();
          s.push(n2);
      }
      // 2. pop
      else if n == "pop" {
          if let Some(x) = s.pop() {
              writeln!(writer,"{}", x).unwrap();
          } else {
            writeln!(writer,"{}", -1).unwrap();
          }
      }
      // 3. size
      else if n == "size" {
        writeln!(writer,"{}", s.len()).unwrap();
      }
      // 4. empty
      else if n == "empty" {
          if s.is_empty() {
            writeln!(writer,"{}", 1).unwrap();
          } else {
            writeln!(writer,"{}", 0).unwrap();
          }
      }
      // 5. top
      else if n == "top" {
          if let Some(&x) = s.last() {
            writeln!(writer,"{}", x).unwrap();
          } else {
            writeln!(writer,"{}", -1).unwrap();
          }
      }
  }
  writer.flush().unwrap();

  
}