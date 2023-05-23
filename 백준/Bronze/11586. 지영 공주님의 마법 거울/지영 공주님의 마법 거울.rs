use std::io::{BufRead, stdin};

fn main() {
    let stdin = stdin();
    let mut reader = stdin.lock();

    // 마법거울의 크기를 입력받음
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // 거울에 비친 지영 공주님의 모습을 입력받음
    let mut princess: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let row: Vec<char> = input.trim().chars().collect();
        princess.push(row);
    }

    // 마법거울의 심리상태를 입력받음
    input.clear();
    reader.read_line(&mut input).unwrap();
    let k: usize = input.trim().parse().unwrap();

    // 마법거울에 비친 지영 공주님의 모습을 출력
    match k {
        1 => {
            for i in 0..n {
                for j in 0..n {
                    print!("{}", princess[i][j]);
                }
                println!();
            }
        }
        2 => {
            for i in 0..n {
                for j in (0..n).rev() {
                    print!("{}", princess[i][j]);
                }
                println!();
            }
        }
        3 => {
            for i in (0..n).rev() {
                for j in 0..n {
                    print!("{}", princess[i][j]);
                }
                println!();
            }
        }
        _ => {
            println!("Invalid mirror state");
        }
    }
}
