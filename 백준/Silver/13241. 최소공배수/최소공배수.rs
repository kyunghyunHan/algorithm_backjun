use std::io::{stdin, BufRead, BufReader, BufWriter, Write};






fn gcd(x: usize, y: usize) -> usize {
  if y == 0 {
      x // y 대신 x를 반환하도록 수정
  } else {
      gcd(y, x % y)
  }
}
fn lcm (x:usize,y:usize)->usize{
  (x * y) / gcd(x, y)
}
fn main() {
  let stdin = stdin();
  let mut reader = BufReader::new(stdin.lock());
  
  let stdout = std::io::stdout();
  let mut writer = BufWriter::new(stdout.lock());
  let mut buffer2 = String::new();
      reader.read_line(&mut buffer2).unwrap(); // 수정된 부분
      let mut nums = buffer2.trim().split_whitespace();
      let x = nums.next().unwrap().parse::<usize>().unwrap();
      let y = nums.next().unwrap().parse::<usize>().unwrap();
      writeln!(writer, "{}", lcm(x, y)).unwrap();
}