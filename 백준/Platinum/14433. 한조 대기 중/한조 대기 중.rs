use std::collections::HashMap;
use std::io;

const N: usize = 310;
type VecInt = Vec<i32>;

fn can(a: usize, adj: &Vec<VecInt>, A: &mut VecInt, B: &mut VecInt, vis: &mut Vec<bool>) -> bool {
    vis[a] = true;
    for &b in &adj[a] {
        if B[b as usize] == 0 || (!vis[B[b as usize] as usize] && can(B[b as usize] as usize, adj, A, B, vis)) {
            A[a] = b as i32;
            B[b as usize] = a as i32;
            return true;
        }
    }
    false
}

fn match_graph(n: usize, k: usize, edges: &[(usize, usize)]) -> usize {
    let mut adj: Vec<VecInt> = vec![VecInt::new(); N];
    let mut A: VecInt = vec![0; N];
    let mut B: VecInt = vec![0; N];
    let mut vis: Vec<bool> = vec![false; N];

    for &(u, v) in edges {
        adj[u].push(v as i32);
    }

    let mut cnt = 0;
    for i in 1..=n {
        if A[i] == 0 {
            vis.iter_mut().for_each(|x| *x = false);
            if can(i, &adj, &mut A, &mut B, &mut vis) {
                cnt += 1;
            }
        }
    }

    cnt
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse values"))
        .collect();

    let n = parts[0];
    let m = parts[1];
    let k1 = parts[2];
    let k2 = parts[3];

    let mut edges: Vec<(usize, usize)> = Vec::new();
    for _ in 0..k1 {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let edge: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse edge"))
            .collect();
        edges.push((edge[0], edge[1]));
    }

    let cnt1 = match_graph(n, k1, &edges);

    edges.clear();
    for _ in 0..k2 {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let edge: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse edge"))
            .collect();
        edges.push((edge[0], edge[1]));
    }

    let cnt2 = match_graph(n, k2, &edges);

    let result = if cnt1 < cnt2 {
        "네 다음 힐딱이"
    } else {
        "그만 알아보자"
    };

    println!("{}", result);
}