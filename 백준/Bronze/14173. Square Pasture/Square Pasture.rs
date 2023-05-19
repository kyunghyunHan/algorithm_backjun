use std::cmp::{max, min};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let pasture1 = parse_pasture(&lines.next().unwrap());
    let pasture2 = parse_pasture(&lines.next().unwrap());

    let min_x = min(pasture1.0, pasture2.0);
    let max_x = max(pasture1.2, pasture2.2);
    let min_y = min(pasture1.1, pasture2.1);
    let max_y = max(pasture1.3, pasture2.3);

    let side = max(max_x - min_x, max_y - min_y);
    let area = side * side;

    println!("{}", area);
}

fn parse_pasture(line: &str) -> (i32, i32, i32, i32) {
    let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
    let x1 = iter.next().unwrap();
    let y1 = iter.next().unwrap();
    let x2 = iter.next().unwrap();
    let y2 = iter.next().unwrap();
    (x1, y1, x2, y2)
}
