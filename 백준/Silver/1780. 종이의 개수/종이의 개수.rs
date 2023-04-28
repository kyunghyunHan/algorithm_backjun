use std::io;

fn paper(x: usize, y: usize, size: usize, map: &mut Vec<Vec<i32>>, one: &mut usize, m_one: &mut usize, ze: &mut usize) {
    let mut same = true;
    for i in y..y+size {
        for j in x..x+size {
            if map[i][j] != map[y][x] {
                same = false;
                break;
            }
        }
        if !same {
            break;
        }
    }
    if !same {
        paper(x, y, size / 3, map, one, m_one, ze);
        paper(x + size / 3, y, size / 3, map, one, m_one, ze);
        paper(x + size / 3 + size / 3, y, size / 3, map, one, m_one, ze);
        paper(x, y + size / 3, size / 3, map, one, m_one, ze);
        paper(x, y + size / 3 + size / 3, size / 3, map, one, m_one, ze);
        paper(x + size / 3, y + size / 3, size / 3, map, one, m_one, ze);
        paper(x + size / 3 + size / 3, y + size / 3 + size / 3, size / 3, map, one, m_one, ze);
        paper(x + size / 3, y + size / 3 + size / 3, size / 3, map, one, m_one, ze);
        paper(x + size / 3 + size / 3, y + size / 3, size / 3, map, one, m_one, ze);
    } else {
        match map[y][x] {
            1 => *one += 1,
            -1 => *m_one += 1,
            _ => *ze += 1,
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut map = vec![vec![0; n]; n];

    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        map[i] = row;
    }

    let mut one = 0;
    let mut m_one = 0;
    let mut ze = 0;

    paper(0, 0, n, &mut map, &mut one, &mut m_one, &mut ze);

    println!("{}\n{}\n{}", m_one, ze, one);
}
