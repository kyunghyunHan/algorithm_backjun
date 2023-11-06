use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::collections::HashSet;

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:i64= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    
    let a: HashSet<i64> = input
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();    
    input.clear();
    reader.read_line(&mut input).unwrap();
    let m:i64= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let v= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i64>>();
    //Vec로는 속도가 느려서 시간제한이 걸림
    //HashSet함수로 사용해서 해야함
    //배열안에 값이 중복이지 않으며 탐색하기에 적합
    for &num in &v {
        //a의 배열안에 v[i]의 값이 잇는지 확인
        if a.contains(&num) {
            //있으면1
            writeln!(writer,"1").unwrap()
        } else {
           //없으면 0

            writeln!(writer,"0").unwrap()
        }
    }

  writer.flush().unwrap();
}