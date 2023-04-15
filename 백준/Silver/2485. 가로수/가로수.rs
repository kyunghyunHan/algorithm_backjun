use std::io;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = n.trim().parse().expect("Invalid input");

    let mut v = Vec::new();
    let mut diff = Vec::new();
    for _i in 0..n {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let x: i32 = input.trim().parse().expect("Invalid input");
        v.push(x);
    }

    for i in 0..n - 1 {
        diff.push(v[(i + 1) as usize] - v[i as usize]);
    }

    let mut g = gcd(diff[0], diff[1]);
    for i in 2..diff.len() {
        g = gcd(g, diff[i]);
    }

    let mut res = 0;
    for i in diff {
        res += i / g - 1;
    }

    println!("{}", res);
}