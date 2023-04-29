use std::io;

fn main() {
    loop {
        let mut ans: i64 = 0;
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.trim() == "#" {
            break;
        }

        let len = input.trim().len();
        let chars: Vec<char> = input.trim().chars().collect();

        for i in 1..=len {
            let tmp: i64;
            match chars[i - 1] {
                '-' => tmp = 0,
                '\\' => tmp = 1,
                '(' => tmp = 2,
                '@' => tmp = 3,
                '?' => tmp = 4,
                '>' => tmp = 5,
                '&' => tmp = 6,
                '%' => tmp = 7,
                '/' => tmp = -1,
                _ => tmp = 0,
            }
            ans += tmp * i64::pow(8, (len - i) as u32);
        }
        println!("{}", ans);
    }
}
