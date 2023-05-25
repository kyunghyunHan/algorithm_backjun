use std::io;

fn main() {
    let mut cnt = 0;

    for i in 1..=8 {
        let mut str = String::new();
        io::stdin().read_line(&mut str).expect("Failed to read line");
        let len = str.trim().len();

        for (j, c) in str.chars().enumerate() {
            if i % 2 == 0 {
                if j % 2 != 0 {
                    if c == 'F' {
                        cnt += 1;
                    }
                }
            } else {
                if j % 2 == 0 {
                    if c == 'F' {
                        cnt += 1;
                    }
                }
            }
        }
    }

    println!("{}", cnt);
}
