use std::io;
use std::cmp;
use std::f64;
use std::collections::HashMap;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input");
    let n: usize = n.trim().parse().expect("Invalid input");

    let mut v: Vec<i32> = Vec::new();
    let mut numv: Vec<(i32, i32)> = Vec::new();

    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input: i32 = input.trim().parse().expect("Invalid input");
        v.push(input);
    }

    if n == 1 {
        let a = v[0];
        let b = v[0];
        let c = v[0];
        let d = 0;
        println!("{}", a);
        println!("{}", b);
        println!("{}", c);
        println!("{}", d);
        return;
    }

    v.sort();

    let mut a = 0;
    for i in 0..n {
        a += v[i];
    }
    a = f64::round(a as f64 / n as f64) as i32;

    let b = v[n / 2];

    let mut num_map: HashMap<i32, i32> = HashMap::new();
    for i in 0..v.len() {
        let count = num_map.entry(v[i]).or_insert(0);
        *count += 1;
    }

    let mut numv: Vec<(i32, i32)> = num_map.into_iter().collect();
    numv.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            b.1.cmp(&a.1)
        }
    });

    let c = if numv[0].1 == numv[1].1 { numv[1].0 } else { numv[0].0 };

    let d = v[n - 1] - v[0];

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
}
