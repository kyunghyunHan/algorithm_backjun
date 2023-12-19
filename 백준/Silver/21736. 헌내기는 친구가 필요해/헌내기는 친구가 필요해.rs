use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
use std::collections::VecDeque;

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());
    let mut  input= String::new();
    reader.read_line(&mut input).unwrap();
    let mut nm= input.trim().split_whitespace();
    let n:usize = nm.next().unwrap().parse().unwrap();
    let m:usize = nm.next().unwrap().parse().unwrap();
  
    let mut campus: Vec<Vec<char>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let  row:Vec<char>= input.trim().chars().collect();
        for j in 0..m {
            if row[j] == 'I' {
                start = (i, j);
            }
        }
        campus.push(row);
    }
    let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let mut res: i32 = 0;
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(start);
    visited[start.0][start.1] = true;

    while let Some((x, y)) = queue.pop_front() {
        for &(dx, dy) in directions.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 {
                let nx_usize = nx as usize;
                let ny_usize = ny as usize;

                if campus[nx_usize][ny_usize] != 'X' && !visited[nx_usize][ny_usize] {
                    queue.push_back((nx_usize, ny_usize));
                    visited[nx_usize][ny_usize] = true;

                    if campus[nx_usize][ny_usize] == 'P' {
                        res += 1;
                    }
                }
            }
        }
    }

    if res > 0 {
        writeln!(writer,"{}", res).unwrap();
    } else {
        writeln!(writer,"TT").unwrap();
    }
    writer.flush().unwrap();
}   


