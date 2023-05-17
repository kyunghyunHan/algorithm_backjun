use std::io::{BufReader,BufWriter,Write,stdin,stdout,BufRead};

fn solve(d1: isize, d2: isize, d3: isize) -> (f32, f32, f32) {
    let b = (d3 + d1 - d2) as f32 / 2.0;
    let c = d3 as f32 - b;
    let a = d1 as f32 - b;

    if a <= 0.0 || b <= 0.0 || c <= 0.0 {
        return (-1.0, -1.0, -1.0);
    }

    (a, b, c)
}

fn main() {
let mut reader= BufReader::new(stdin().lock());
let mut input = String::new();

reader.read_line(&mut input).unwrap();
let mut nums= input.trim().split_whitespace().map(|x|x.parse::<isize>().unwrap())
;   

    let d1: isize = nums.next().unwrap();
    let d2: isize = nums.next().unwrap();
    let d3: isize = nums.next().unwrap();

    let ans = solve(d1, d2, d3);

    if ans.0 == -1.0 {
        println!("-1");
    } else {
        println!("1");
        println!("{:.1} {:.1} {:.1}", ans.0, ans.1, ans.2);
    }
}
