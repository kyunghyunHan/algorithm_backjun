use std::{io::{BufReader,BufRead,BufWriter,Write,stdin,stdout}, vec};
use std::collections::VecDeque;

fn bfs(graph:&mut Vec<Vec<usize>>,start:usize,n:usize)->usize{
    let mut num=vec![0;n+1];
    let mut visited =vec![start];
    let mut queue:VecDeque<usize>= VecDeque::new();
     queue.push_back(start);
     while let Some(a) = queue.pop_front() {
        for &i in &graph[a] {
            if !visited.contains(&i) {
                num[i] = num[a] + 1;
                visited.push(i);
                queue.push_back(i);
            }
        }
     }

    
    num.iter().sum()
}
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();

    let nm:Vec<usize>= input. trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let n= nm[0];
    let m= nm[1];

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for i in 0..m{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let ab:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
        let a= ab[0];
        let b= ab[1];
        graph[a].push(b);
        graph[b].push(a)
    }
    let mut result:Vec<usize>=vec![];

    for i in 1..n+1{
        result.push(bfs(&mut graph,i,n))
    }
    if let Some(min_index) = result.iter().enumerate().min_by_key(|(_, &val)| val) {
        writeln!(writer,"{}", min_index.0 + 1).unwrap();
    }
    writer.flush().unwrap();

}
