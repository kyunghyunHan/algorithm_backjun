use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::collections::VecDeque;
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let inputs:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let n= inputs[0];
    let k = inputs[1];
    let mut result:i32=0;
    let mut q:VecDeque<(i32,i32)>= VecDeque::new();
    q.push_back((n,0));
    let mut visit :Vec<bool>= vec![false;1000001];

    visit[n as usize]= true;

    bfs(n, &mut q, k, &mut result, &mut visit);


   
   writeln!(writer,"{}",result).unwrap();
   writer.flush().unwrap();



}
fn bfs(n:i32,q:&mut VecDeque<(i32,i32)>,k:i32,result:&mut i32,visit:&mut Vec<bool>)->i32{
    while !q.is_empty(){
        let data= q.front().unwrap().0;
        let depth= q.front().unwrap().1;
        q.pop_front();

        if data==k{
              *result=depth;
              break;

        }
        if valid(data - 1, &visit) {
            visit[data as usize - 1] = true;
            q.push_back((data - 1, depth + 1));
        }
        if valid(data + 1, &visit) {
            visit[data as usize + 1] = true;
            q.push_back((data + 1, depth + 1));
        }
        if valid(data * 2, &visit) {
            visit[data as usize * 2] = true;
            q.push_back((data * 2, depth + 1));
        }


    }
    0

}
fn valid(n: i32, visit: &[bool]) -> bool {
    n >= 0 && n <= 100000 && !visit[n as usize]
}
