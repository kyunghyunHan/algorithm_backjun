use std::io;

fn main() {
    let mut input_line = String::new();
    while io::stdin().read_line(&mut input_line).unwrap() > 0 {
        let mut split_iter = input_line.split_whitespace();
        let n: i32 = split_iter.next().unwrap().parse().expect("Failed to parse n");
        let k: i32 = split_iter.next().unwrap().parse().expect("Failed to parse k");

        if k > 1 && k <= n && n <= 1_000_000_000 {
            let mut sum = n;
            let mut n_mut = n;
            while n_mut >= k {
                let re = n_mut % k;
                n_mut = n_mut / k;
                sum += n_mut;
                n_mut += re;
            }
            println!("{}", sum);
        }

        input_line.clear();
    }
}
