use std::io;

fn main() {
    let mut cnt: [[i32; 1010]; 1010] = [[0; 1010]; 1010];
    let mut student: [[i32; 10]; 1010] = [[0; 10]; 1010];

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    for i in 1..=n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choices: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid input"))
            .collect();

        for j in 1..=5 {
            student[i][j] = choices[j - 1];
        }
    }

    for k in 1..=5 {
        for i in 1..=n {
            for j in 1..=n {
                if i == j {
                    continue;
                }
                if student[i][k] == student[j][k] {
                    cnt[i][j] = 1;
                }
            }
        }
    }

    let mut maxcnt = 0;
    let mut res = 1;
    for i in 1..=n {
        let mut c = 0;
        for j in 1..=n {
            c += cnt[i][j];
        }
        if maxcnt < c {
            maxcnt = c;
            res = i;
        }
    }

    println!("{}", res);
}
