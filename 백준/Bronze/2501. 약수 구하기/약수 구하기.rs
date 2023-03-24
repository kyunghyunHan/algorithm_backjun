use std::io;

fn main() {
    let mut ans: [i32; 10000] = [10000; 10000];
    let mut cnt = 0;
    let mut p = 0;
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let mut nums = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());

    let n = nums.next().unwrap();
    let k = nums.next().unwrap();

    for i in 1..10000 {
        if i > n {
            break;
        }
        if n % i == 0 {
            cnt += 1;
            ans[p] = i;
            p += 1;
        }
    }

    if cnt < k {
        println!("0");
    } else {
        println!("{}", ans[k as usize - 1]);
    }
}