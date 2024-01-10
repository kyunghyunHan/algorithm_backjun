use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
// DFS function
fn dfs(
    x: usize,
    y: usize,
    sum_value: i32,
    length: i32,
    n: usize,
    m: usize,
    a: &Vec<Vec<i32>>,
    check: &mut Vec<Vec<bool>>,
    result: &mut i32,
) {
    if length >= 4 {
        *result = std::cmp::max(*result, sum_value);
        return;
    }

    let dx: [i32; 4] = [0, 0, 1, -1];
    let dy: [i32; 4] = [-1, 1, 0, 0];

    for i in 0..4 {
        let nx = x as i32 + dx[i];
        let ny = y as i32 + dy[i];

        if nx < 1 || nx > n as i32 || ny < 1 || ny > m as i32 {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if !check[nx][ny] {
            check[nx][ny] = true;
            dfs(
                nx,
                ny,
                sum_value + a[nx][ny],
                length + 1,
                n,
                m,
                a,
                check,
                result,
            );
            check[nx][ny] = false;
        }
    }
}

// Function to check special shape 'ㅏ', 'ㅓ', 'ㅗ', 'ㅜ'
fn check_exshape(
    x: usize,
    y: usize,
    n: usize,
    m: usize,
    a: &Vec<Vec<i32>>,
    result: &mut i32,
) {
    let ex: Vec<Vec<i32>> = vec![
        vec![0, 0, 0, 1],
        vec![0, 1, 2, 1],
        vec![0, 0, 0, -1],
        vec![0, -1, 0, 1],
    ];
    let ey: Vec<Vec<i32>> = vec![
        vec![0, 1, 2, 1],
        vec![0, 0, 0, 1],
        vec![0, 1, 2, 1],
        vec![0, 1, 1, 1],
    ];

    for i in 0..4 {
        let mut is_out = false;
        let mut sum_value = 0;

        for j in 0..4 {
            let nx = x as i32 + ex[i][j];
            let ny = y as i32 + ey[i][j];

            if nx < 1 || nx > n as i32 || ny < 1 || ny > m as i32 {
                is_out = true;
                break;
            } else {
                let nx = nx as usize;
                let ny = ny as usize;
                sum_value += a[nx][ny];
            }
        }

        if !is_out {
            *result = std::cmp::max(*result, sum_value);
        }
    }
}

fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let nm: Vec<usize> = input.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = nm[0];
    let m = nm[1];

    let mut a: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
    let mut check: Vec<Vec<bool>> = vec![vec![false; m + 1]; n + 1];
    let mut result = 0;

    for i in 1..=n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let nums: Vec<i32> =  input.trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        for j in 1..=m {
            a[i][j] = nums[j - 1];
        }
    }

    for i in 1..=n {
        for j in 1..=m {
            check[i][j] = false;
        }
    }

    for i in 1..=n {
        for j in 1..=m {
            check[i][j] = true;
            dfs(
                i,
                j,
                a[i][j],
                1,
                n,
                m,
                &a,
                &mut check,
                &mut result,
            );
            check[i][j] = false;

            check_exshape(i, j, n, m, &a, &mut result);
        }
    }

    writeln!(writer,"{}", result).unwrap();
    writer.flush().unwrap();
}
