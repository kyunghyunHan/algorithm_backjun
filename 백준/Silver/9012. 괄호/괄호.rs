use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse().unwrap();
    for _  in 0..n{
      input.clear();
      reader.read_line(&mut input).unwrap();
      let s= input.trim();
      if is_balanced(&s) {
        writeln!(writer,"YES").unwrap();
    } else {
      writeln!(writer,"NO").unwrap();
    }
    }
    writer.flush().unwrap();
}
fn is_balanced(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    //문자열 순회
    for ch in s.chars() {
      //스택이 비어있어야함
      //(이거면 추가
        if ch == '(' {
            stack.push(ch);
             //)이거면 삭제
        } else if ch == ')' {
          //Vec이 비어있으면 순회가 끝나기 전에 비어 있으면 한쪽으로 기울엇다 판단하고 false
            if stack.is_empty() {
                return false;
            } else {
                stack.pop();
            }
        }
    }
    //Vec이 비어있으면 true를 반환
    stack.is_empty()
}