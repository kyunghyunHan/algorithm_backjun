use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn dfs(v: usize, visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>) {
    visited[v] = true;
    for &i in &graph[v] {
        if !visited[i] {
            dfs(i, visited, graph);
        }
    }
}

fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();    
    reader.read_line(&mut input).unwrap();
    let  inputs:Vec<usize>= input.trim().split_ascii_whitespace().map(|x|x.parse::<usize>().unwrap()).collect();
    let n: usize = inputs[0];
    let m: usize =inputs[1];

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n  + 1];
    let mut visited: Vec<bool> = vec![false; n  + 1];

    for _ in 0..m {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let  inputs:Vec<usize>= input.trim().split_ascii_whitespace().map(|x|x.parse::<usize>().unwrap()).collect();
        let s: usize = inputs[0];
        let e: usize =inputs[1];
    
        graph[s ].push(e);
        graph[e ].push(s);
    }

    let mut count = 0;
    for i in 1..=n {
        if !visited[i ] {
            count += 1;
            dfs(i , &mut visited, &graph);
        }
    }

    writeln!(writer,"{}", count).unwrap();
    writer.flush().unwrap();
}
