use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let  nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let n= nums[0];
    let m= nums[1];
    let v= nums[2];
    let mut  graph =vec![vec![0;nums[0]+1];nums[0]+1];
    //그래프 만들기
    for _ in 0..m{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let  ab= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<usize>>();
        let a= ab[0];
        let b= ab[1];

        graph[a][b]= 1;
        graph[b][a]=1;
    }

    //방문 리스트 행렬
    let mut visited1: Vec<usize> = vec![0; n+1];
    let mut visited2: Vec<usize> = vec![0; n + 1];
    dfs(v,n,&mut graph,&mut visited1,&mut writer);
    writeln!(writer).unwrap();
    bfs(v,n,&mut graph,&mut visited2,&mut writer);
}    

fn dfs(v:usize,n:usize, graph:&mut Vec<Vec<usize>>,visited1:&mut Vec<usize>,writer:&mut BufWriter<std::io::StdoutLock<'_>>){
    visited1[v]=1;//방문 처리
    write!(writer,"{} ",v).unwrap();
    for i in 1..=n {
        if graph[v][i] == 1  && visited1[i]==0{
            dfs(i,n,graph,visited1,writer);
        }
    }
}
fn bfs (v:usize,n:usize, graph:&mut Vec<Vec<usize>>,visited2:&mut Vec<usize>,writer:&mut BufWriter<std::io::StdoutLock<'_>>){
    let mut queue: Vec<usize> = Vec::new();
     queue.push(v);
     visited2[v]=1;
     while !queue.is_empty() {
        let node = queue.remove(0);

        write!(writer,"{} ",node).unwrap();
        for i in 1..=n {
            if visited2[i]==0 && graph[node][i]==1{
             queue.push(i);
            visited2[i]=1;
        }
    }
}
}