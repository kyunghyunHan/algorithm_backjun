use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    let mut ans: [i32; 7] = [0; 7];
    let mut p = 0;
    let mut dice: [[i32; 5]; 1001] = [[0; 5]; 1001];

    for i in 1..=n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        dice[i as usize] = [0, numbers[0], numbers[1], numbers[2], numbers[3]];
    }

    for i in 1..=n {
        ans = [0; 7];
        for j in 1..=4 {
            for c in 1..=6 {
                if dice[i as usize][j as usize] == c {
                    ans[c as usize] += 1;
                }
            }
        }

        for i in 1..=6 {
            let mut ting = 0;
            if ans[i as usize] == 4 {
                if p < 50000 + i * 5000 {
                    p = 50000 + i * 5000;
                }
                break;
            } else if ans[i as usize] == 3 {
                if p < 10000 + i * 1000 {
                    p = 10000 + i * 1000;
                }
                break;
            } else if ans[i as usize] == 2 {
                let mut ck = 1;
                let mut i2 = 0;
                for t in 1..=6 {
                    if ans[t as usize] == 2 && t != i {
                        i2 = t;
                        ck += 1;
                    }
                }
                if ck == 2 {
                    if p < 2000 + i * 500 + i2 * 500 {
                        p = 2000 + i * 500 + i2 * 500;
                    }
                } else {
                    if p < 1000 + i * 100 {
                        p = 1000 + i * 100;
                    }
                }
                break;
            }

            for i in 1..=6 {
                if ans[i as usize] > 0 && ting < i {
                    ting = i;
                }
            }

            if p < ting * 100 {
                p = ting * 100;
            }
        }
    }

    println!("{}", p);
}
