use std::io;
fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let n: Vec<usize> = input_a
        .split_whitespace()
        .map(|x| -> usize { x.parse().unwrap() })
        .collect();

    let mut ans = 0;
    let mut cnt = 0;
    let mut temp;
    while cnt != n[0] {
        ans += 1;
        temp = ans;
        while temp != 0 {
            if temp % 1000 == 666 {
                cnt += 1;
                break;
            } else {
                temp /= 10;
            }
        }
    }
    println!("{}", ans)
}
