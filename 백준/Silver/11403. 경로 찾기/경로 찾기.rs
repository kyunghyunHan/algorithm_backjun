use std::io::{self, BufRead};

fn dfs(adj_list: &Vec<Vec<usize>>, visit: &mut Vec<usize>, x: usize) {
    for &node in &adj_list[x] {
        if visit[node] == 0 {
            visit[node] = 1;
            dfs(adj_list, visit, node);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    for i in 0..n {
        let values: Vec<usize> = lines
            .next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        for (j, &value) in values.iter().enumerate() {
            if value == 1 {
                graph[i].push(j);
            }
        }
    }

    for i in 0..n {
        let mut visit = vec![0; n];
        dfs(&graph, &mut visit, i);

        for &visited in &visit {
            print!("{} ", visited);
        }
        println!();
    }
}
