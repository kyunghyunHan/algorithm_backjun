use std::io::{stdin, BufRead, BufReader, BufWriter, Write};

fn main() {
  let stdin = stdin();
  let mut reader = BufReader::new(stdin.lock());
  
  let stdout = std::io::stdout();
  let mut writer = BufWriter::new(stdout.lock());


  let mut buffer = String::new();
      reader.read_line(&mut buffer).unwrap(); // 수정된 부분
      let mut nums = buffer.trim().split_whitespace();
      let x = nums.next().unwrap().parse::<usize>().unwrap();
      let y = nums.next().unwrap().parse::<usize>().unwrap();
  let mut buffer2 = String::new();
      reader.read_line(&mut buffer2).unwrap(); // 수정된 부분
      let mut nums = buffer2.trim().split_whitespace();
      let x2 = nums.next().unwrap().parse::<usize>().unwrap();
      let y2 = nums.next().unwrap().parse::<usize>().unwrap();
      
     let r1= x2*y+x*y2;
      let r2= y2*y;
      
      writeln!(writer,"{} {}",r1/gcd(r1,r2),r2/gcd(r1,r2)).unwrap();
}
fn gcd(x: usize, y: usize) -> usize {
  if y != 0 {
      gcd(y, x % y)
  } else {
      x
  }
}