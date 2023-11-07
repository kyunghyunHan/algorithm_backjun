use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::collections:: VecDeque;
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    //n장의 카
    let n:u64= input.trim().parse().unwrap();
    //1번 카드가 젤  위에
    //n번 카드가 젤 아래
    let mut v: VecDeque<u64> = (1..=n).collect();
    while v.len()>1 {
        if  v.len()<1 {
        writeln!(writer,"{}",v.front().unwrap()).unwrap();
        break;

       }
       //맨 앞 삭제
       v.pop_front();
       //삭제 후 
       let front = v.pop_front().unwrap();
       //뒤에 집어넣기
       v.push_back(front);
    }
    writeln!(writer,"{}",v.front().unwrap()).unwrap();

  writer.flush().unwrap();
}