use std::io::{BufReader, BufRead, BufWriter, Write, stdin, stdout};
use std::collections::VecDeque;

fn main() {
    let mut reader = BufReader::new(stdin());
    let mut writer = BufWriter::new(stdout());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let nm: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = nm[0];
    let m = nm[1];
    let mut graph = Vec::new();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s: Vec<usize> = input.trim().chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
        graph.push(s);
    }
    let dx = [-1, 1, 0, 0];
    let dy = [0, 0, -1, 1];

    writeln!(writer, "{}", bfs(0, 0, &dx, &dy, n as i32, m as i32, &mut graph)).unwrap();
    writer.flush().unwrap();
}

fn bfs(x: usize, y: usize, dx: &[i32], dy: &[i32], n: i32, m: i32, graph: &mut Vec<Vec<usize>>) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    while let Some((x, y)) = queue.pop_front() {
        for i in 0..4 {
            let nx = x as i32 + dx[i];
            let ny = y as i32 + dy[i];

            if nx >= 0 && ny >= 0 && nx < n && ny < m && graph[nx as usize][ny as usize] == 1 {
                queue.push_back((nx as usize, ny as usize));
                graph[nx as usize][ny as usize] = graph[x][y] + 1;
            }
        }
    }
    return graph[n as usize - 1][m as usize - 1];
}
