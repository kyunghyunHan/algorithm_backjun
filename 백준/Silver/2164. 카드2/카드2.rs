use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::collections::VecDeque;

fn main(){
    let mut reader= BufReader::new(stdin().lock());

    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
let mut writer= BufWriter::new(stdout().lock());
    let mut n= nums.next().unwrap();
    
    let mut q: VecDeque<usize> = (1..=n).collect();




   while q.len()>1 {
    q.pop_front();
    let front = q.pop_front().unwrap();

   q.push_back(front);
       
   }
   writeln!(writer,"{}",q.front().unwrap()).unwrap();
}
//요세푸스문제
//1부터 n까지 q에 넣고 맨앞의 수를 뺴서 맨뒤로 보내는 과정을 반복하여 마닞막에 큐에 남는 수를 출력