use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
  let  n= input.trim();
  let mut answer= 0;

  let m:Vec<&str>= n.split('-').collect();

 let parts: Vec<&str> = m[0].split('+').collect();
 let x: i32 = parts.iter().map(|&s| s.parse::<i32>().unwrap_or(0)).sum();
 if input.trim().chars().next().unwrap() == '-' {
  answer -= x;
} else {
  answer += x;
}

for x in m.iter().skip(1) {
  let parts: Vec<&str> = x.split('+').collect();
  let sum: i32 = parts.iter().map(|&s| s.parse::<i32>().unwrap_or(0)).sum();
  answer -= sum;
}

writeln!(writer,"{}",answer).unwrap();
writer.flush().unwrap();
}