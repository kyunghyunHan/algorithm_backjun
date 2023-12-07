use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main() {
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input = String::new();

   reader.read_line(&mut input).unwrap();
   let t: usize = input.trim().parse().unwrap(); // t를 변수로 선언 (let 사용)
   input.clear();
    reader.read_line(&mut input).unwrap();
    let mut sub: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    while sub.len() < 5 {
        sub.push(0);
    }

    let mut res = 0;

    if sub[0] > sub[2] {
        res += (sub[0] - sub[2]) * 508;
    } else {
        res += (sub[2] - sub[0]) * 108;
    }

    if sub[1] > sub[3] {
        res += (sub[1] - sub[3]) * 212;
    } else {
        res += (sub[3] - sub[1]) * 305;
    }

    if sub[4] > 0 {
        res += sub[4] * 707;
    }

    res *= 4763;

    writeln!(writer,"{}", res).unwrap();
}  