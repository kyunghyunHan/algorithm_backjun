//ACM호텔
use std::io;
use std::str;

//h w n
fn main() {
    //테스트데이터
    let mut input_one = String::new();
    io::stdin().read_line(&mut input_one).unwrap();
    let v1: Vec<i32> = input_one
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let T = v1[0];
    let mut H1 = 0;
    let mut W1 = 0;
    for i in 0..T {
        let mut input_Two = String::new();
        io::stdin().read_line(&mut input_Two).unwrap();
        let mut v2: Vec<i32> = input_Two
            .split_whitespace()
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect();

        H1 = v2[2] % v2[0];
        W1 = v2[2] / v2[0];

        if H1 > 0 {
            W1 += 1;
        } else {
            H1 = v2[0]
        }

        println!("{}", H1 * 100 + W1)
    }
}
