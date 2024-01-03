use std::io::{BufReader, BufRead, BufWriter, Write, stdin, stdout};
use std::collections::VecDeque;

fn main() {
    let mut reader = BufReader::new(stdin());
    let mut writer = BufWriter::new(stdout());
    let mut input = String::new();
    let mut graph: Vec<Vec<usize>>= vec![];
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s: Vec<usize> = input.trim().chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
        graph.push(s);
    }
    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];
    let mut cnt: Vec<usize> = vec![];
    for i in 0..n {
        for j in 0..n {
            if graph[i][j] == 1 {
                cnt.push(bfs(&dx, &dy, i as i32, j as i32, &mut graph));
            }
        }
    }
    cnt.sort();
    writeln!(writer, "{}", cnt.len()).unwrap();
    for i in cnt {
        writeln!(writer, "{}", i).unwrap();
    }
    writer.flush().unwrap();
}

fn bfs(dx: &[i32], dy: &[i32], a: i32, b: i32, graph: &mut Vec<Vec<usize>>) -> usize {
    let n = graph.len() as i32;
    let mut queue = VecDeque::new();
    queue.push_back((a, b));
    graph[a as usize][b as usize] = 0;
    let mut count = 1;

    while let Some((x, y)) = queue.pop_front() {
        for i in 0..4 {
            let nx = x + dx[i as usize];
            let ny = y + dy[i as usize];
            if nx < 0 || nx >= n || ny < 0 || ny >= n {
                continue;
            } else if graph[nx as usize][ny as usize] == 1 {
                graph[nx as usize][ny as usize] = 0;
                queue.push_back((nx, ny));
                count += 1;
            }
        }
    }
    count
}
