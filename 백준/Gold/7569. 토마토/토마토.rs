use std::collections::{VecDeque, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

fn main() {
    let directions = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let m = iter.next().unwrap();
    let n = iter.next().unwrap();
    let h = iter.next().unwrap();

    let mut grid = vec![];
    let mut fresh_count = 0;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    for _ in 0..h {
        let mut layer = vec![];
        for _ in 0..n {
            let mut row = vec![];
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let row_input: Vec<isize> = input
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            for val in row_input {
                if val == 1 {
                    queue.push_back(Point { x: layer.len(), y: row.len(), z: grid.len() });
                    visited.insert((layer.len(), row.len(), grid.len()));
                } else if val == 0 {
                    fresh_count += 1;
                }
                row.push(val);
            }
            layer.push(row);
        }
        grid.push(layer);
    }

    let mut days = 0;
    while let Some(p) = queue.pop_front() {
        let (x, y, z) = (p.x, p.y, p.z);

        for &(dx, dy, dz) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            let nz = z as isize + dz;

            if nx >= 0 && ny >= 0 && nz >= 0 && nx < n as isize && ny < m as isize && nz < h as isize {
                let (nx, ny, nz) = (nx as usize, ny as usize, nz as usize);

                if !visited.contains(&(nx, ny, nz)) && grid[nz][nx][ny] == 0 {
                    visited.insert((nx, ny, nz));
                    queue.push_back(Point { x: nx, y: ny, z: nz });
                    grid[nz][nx][ny] = grid[z][x][y] + 1;
                    days = grid[nz][nx][ny] - 1;
                    fresh_count -= 1;
                }
            }
        }
    }

    if fresh_count > 0 {
        println!("-1");
    } else {
        println!("{}", days);
    }
}
