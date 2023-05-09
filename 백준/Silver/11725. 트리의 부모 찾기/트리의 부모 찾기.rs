use std::{collections::VecDeque, usize};
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
const MAX: usize = 100001;

fn bfs(graph: &Vec<Vec<usize>>, ans: &mut [usize], visit: &mut [bool]) {
    let mut q: VecDeque<usize> = VecDeque::new();
    visit[1] = true;
    q.push_back(1);

    while let Some(parent) = q.pop_front() {
        for &child in &graph[parent] {
            if !visit[child] {
                ans[child] = parent;
                visit[child] = true;
                q.push_back(child);
            }
        }
    }
}

fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); MAX];
    let mut visit: [bool; MAX] = [false; MAX];
    let mut ans: [usize; MAX] = [0; MAX];
   let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n-1 {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        graph[x].push(y);
        graph[y].push(x);
    }

    bfs(&graph, &mut ans, &mut visit);

    for i in 2..=n {
        writeln!(writer,"{}", ans[i]).unwrap();
    }
}
