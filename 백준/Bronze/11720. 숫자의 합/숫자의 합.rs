use std::io;
fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let v: Vec<usize> = input_a
        .split_whitespace()
        .map(|x| -> usize { x.parse().unwrap() })
        .collect();

    let n = v[0];
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).unwrap();
    let commend: Vec<&str> = input_b.split_whitespace().collect();
    let test = commend[0];
    let mut result: usize = 0;
    for i in 0..n {
        let s = &test[i..i + 1];
        let my_int: usize = s.parse().unwrap();
        result += my_int;
    }
    println!("{}", result);
}
