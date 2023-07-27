use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let sx: i32 = iter.next().expect("Invalid input").parse().expect("Invalid number");
    let sy: i32 = iter.next().expect("Invalid input").parse().expect("Invalid number");

    let mut ans: i32 = 0;
    let mut x: i32 = sx;
    let mut y: i32 = sy;

    for _ in 0..n - 1 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let mut iter = input.split_whitespace();
        let nx: i32 = iter.next().expect("Invalid input").parse().expect("Invalid number");
        let ny: i32 = iter.next().expect("Invalid input").parse().expect("Invalid number");

        ans += (nx + ny - x - y).abs();
        x = nx;
        y = ny;
    }

    ans += (x + y - sx - sy).abs();
    println!("{}", ans);
}
