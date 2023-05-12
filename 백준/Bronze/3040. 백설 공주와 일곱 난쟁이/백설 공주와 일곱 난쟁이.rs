use std::io;

const MAX: usize = 9;

fn main() {
    let mut a: [i32; MAX] = [0; MAX];
    let mut acc: i32 = 0;

    for i in 0..MAX {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let n = input.trim().parse().expect("Invalid input");
        a[i] = n;
        acc += n;
    }

    a.sort();

    for i in 0..MAX - 1 {
        for j in i + 1..MAX {
            if acc - a[i] - a[j] == 100 {
                for k in 0..MAX {
                    if a[k] != a[i] && a[k] != a[j] {
                        println!("{}", a[k]);
                    }
                }
                std::process::exit(0);
            }
        }
    }
}
