use std::io;

fn main() {
    let mut cnt = 0;
    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if n == 0 {
            break;
        }
        for i in (n + 1)..=(2 * n) {
            if i == 2 || i == 3 {
                cnt += 1;
                continue;
            }
            if i % 2 == 1 {
                let rt = (i as f64).sqrt() as u32;
                let mut is_prime = true;
                for j in 2..=rt {
                    if i % j == 0 {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    cnt += 1;
                }
            }
        }
        println!("{}", cnt);
        cnt = 0;
    }
}
