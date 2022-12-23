use std::cmp::{max, min};
use std::io::{stdin, Read, Write, stdout, BufWriter};

const MATRIX_SIZE: usize = 21;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut matrix = [[[1; MATRIX_SIZE]; MATRIX_SIZE]; MATRIX_SIZE];

    for a in 1..=20 {
        for b in 1..=20 {
            for c in 1..=20 {
                if a < b && b < c {
                    matrix[a][b][c] = matrix[a][b][c-1] + matrix[a][b-1][c-1] - matrix[a][b-1][c];
                } else {
                    matrix[a][b][c] = matrix[a-1][b][c] + matrix[a-1][b-1][c] + matrix[a-1][b][c-1] - matrix[a-1][b-1][c-1];
                }
            }
        }
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    loop {
        let a = input.next().unwrap();
        let b = input.next().unwrap();
        let c = input.next().unwrap();
        if a == -1 && b == -1 && c == -1 {
            break;
        }
        writeln!(writer, "w({a}, {b}, {c}) = {}",
            if a <= 0 || b <= 0 || c <= 0 {
                matrix[0][0][0]
            } else if a > 20 || b > 20 || c > 20 {
                matrix[20][20][20]
            } else {
                matrix[a as usize][b as usize][c as usize]
            }
        ).unwrap();
    }
}