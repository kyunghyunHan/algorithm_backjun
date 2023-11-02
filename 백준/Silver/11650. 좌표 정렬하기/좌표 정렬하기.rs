use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input: String = String::new();
  reader.read_line(&mut input).unwrap();
  //점의 개수
  let n:i32= input.trim().parse().unwrap();

  //배열선언
  let mut  v:Vec<(i32,i32)>= Vec::new();
  for _ in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let nums:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    //좌표값 배열에 집어 넣기
    v.push((nums[0],nums[1]))
  }

  v.sort_by(|a,b|{
    if a.0==b.0{
      a.1.cmp(&b.1)
    }else{
      a.0.cmp(&b.0)
    }
  });
  for (a, b) in v {
    writeln!(writer, "{} {}", a, b).unwrap();
}

   writer.flush().unwrap();

}