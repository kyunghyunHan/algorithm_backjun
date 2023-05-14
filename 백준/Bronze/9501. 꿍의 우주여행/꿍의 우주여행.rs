use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let t: i32 = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut values = line.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let n = values.next().unwrap();
        let d = values.next().unwrap();

        let mut ans = 0;

        for _ in 0..n {
            let line = lines.next().unwrap();
            let mut values = line.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
            let vi = values.next().unwrap();
            let fi = values.next().unwrap();
            let ci = values.next().unwrap();

            if vi * fi >= ci * d {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}
