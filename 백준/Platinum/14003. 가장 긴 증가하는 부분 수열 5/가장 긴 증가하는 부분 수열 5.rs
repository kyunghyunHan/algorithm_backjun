use std::cmp::Ordering;

fn main() {
    let mut num = Vec::new();
    let mut dp = Vec::new();
    let mut L = Vec::new();
    let mut s = Vec::new();
    let mut idx = 0;
    let mut dptmp = 0;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let N: usize = input.trim().parse().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num_str: Vec<&str> = input.trim().split_whitespace().collect();
    for i in 0..N {
        let a: i32 = num_str[i].parse().unwrap();
        num.push(a);
        if L.len() == 0 {
            L.push(num[i]);
            dp.push(1);
        } else {
            if L[L.len() - 1] < num[i] {
                L.push(num[i]);
                dp.push(L.len() as i32);
            } else {
                let p = L.binary_search_by(|&x| x.cmp(&num[i])).unwrap_or_else(|x| x);
                L[p] = num[i];
                dp.push((p + 1) as i32);
            }
        }
        if dp[i] > dptmp {
            idx = i;
            dptmp = dp[i];
        }
    }
    println!("{}", L.len());
    s.push(num[idx]);
    for i in (0..idx).rev() {
        if num[i] < num[idx] && dp[i] + 1 == dp[idx] {
            idx = i;
            s.push(num[i]);
        }
    }
    println!();
    while !s.is_empty() {
        print!("{} ", s.pop().unwrap());
    }
}