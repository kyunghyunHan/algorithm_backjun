use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
fn main() {
     let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
  
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap(); // 수정된 부분
    let mut nums = buffer.trim().split_whitespace();
    let n = nums.next().unwrap().parse::<usize>().unwrap();
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let mut s: Vec<usize> = vec![];
    for i in 0..n {
        let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap(); // 수정된 부분
    let mut nums = buffer.trim().split_whitespace();
    let n = nums.next().unwrap().parse::<usize>().unwrap();
  

    if n ==0{
        s.pop();
    }
    else{
        s.push(n);
    }




    }
   
   let mut result =0;
   for i in s{
       result +=i;
   }

   writeln!(writer,"{}",result).unwrap();

}
