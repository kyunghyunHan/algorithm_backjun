use std::io;

fn main() {
    loop {
        let mut input_a = String::new();
        io::stdin().read_line(&mut input_a).unwrap();
        let v: Vec<i32> = input_a
            .split_whitespace()
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect();
        let x = v[0];
        let y = v[1];
        let z = v[2];

        let mut st = 0;
        let mut nd = 0;
        let mut rd = 0;

        if x >= y {
            if y >= z {
                st = x;
                nd = y;
                rd = z;
            }
            if x >= z {
                st = x;
                nd = z;
                rd = y;
            }
            if z >= x {
                st = z;
                nd = x;
                rd = y;
            }
        }
        if y >= x {
            if x >= z {
                st = y;
                nd = x;
                rd = z;
            }
            if y >= z {
                st = y;
                nd = z;
                rd = x;
            }
            if z >= y {
                st = z;
                nd = y;
                rd = x;
            }
        }
        if st == 0 || nd == 0 || rd == 0 {
            break;
        }
        if (st * st) == (nd * nd) + (rd * rd) {
            println!("{}", "right");
        } else {
            println!("{}", "wrong");
        }
    }
}
