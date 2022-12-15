use std::io;

fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let v: Vec<i32> = input_a
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).unwrap();
    let v2: Vec<i32> = input_b
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let mut input_c = String::new();
    io::stdin().read_line(&mut input_c).unwrap();
    let v3: Vec<i32> = input_c
        .split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect();
    let x1 = v[0];
    let y1 = v[1];
    let x2 = v2[0];
    let y2 = v2[1];
    let x3 = v3[0];
    let y3 = v3[1];

    if x1 == x2 {
        if y1 == y2 {
            println!("{} {}", x3, y3);
        } else if y1 == y3 {
            println!("{} {}", x3, y2);
        } else {
            println!("{} {}", x3, y1)
        }
    } else if x1 == x3 {
        if y1 == y2 {
            println!("{} {}", x2, y3);
        } else if y1 == y3 {
            println!("{} {}", x2, y2);
        } else {
            println!("{} {}", x2, y1)
        }
    } else {
        if y1 == y2 {
            println!("{} {}", x1, y3);
        } else if y1 == y3 {
            println!("{} {}", x1, y2);
        } else {
            println!("{} {}", x1, y1)
        }
    }
}
