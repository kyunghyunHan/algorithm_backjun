use std::io;

fn main() {
    let mut n_input = String::new();
    io::stdin().read_line(&mut n_input).unwrap();
    let n: usize = n_input.trim().parse().unwrap();

    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();

    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for c in str.chars() {
        match c {
            '2' => cnt1 += 1,
            'e' => cnt2 += 1,
            _ => {}
        }
    }

    if cnt1 == cnt2 {
        println!("yee");
    } else if cnt1 > cnt2 {
        println!("2");
    } else {
        println!("e");
    }
}
