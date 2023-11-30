use std::io ::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
use std::collections::VecDeque;


fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    //컴퓨터 개수
    let n:usize= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    //연결선 개수
    let v:usize= input.trim().parse().unwrap();
    
     //그래프  초기화
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    //방문한 컴퓨터인지 확인
    let mut visited: Vec<usize> = vec![0; n + 1];
    
    for _ in 0..v {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let nums: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let a = nums[0];//a에 b연결
        let b = nums[1];//b에 a연결 -> 양방향
        graph[a].push(b);
        graph[b].push(a);
    }
     //1번 컴퓨터
    visited[1] = 1;
    let mut queue = VecDeque::new();
    queue.push_back(1);

    while let Some(c) = queue.pop_front() {
        for &nx in &graph[c] {
            if visited[nx] == 0 {
                queue.push_back(nx);
                visited[nx] = 1;
            }
        }
    }

    let total_visited = visited.iter().sum::<usize>() - 1;
    writeln!(writer,"{}", total_visited).unwrap();
}