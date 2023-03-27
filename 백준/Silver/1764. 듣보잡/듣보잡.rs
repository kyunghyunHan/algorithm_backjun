use std::collections::HashMap;
use std::vec::Vec;

fn main() {
    let mut ma: HashMap<String, i32> = HashMap::new();
    let mut vt: Vec<String> = Vec::new();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("read error");
    let nm: Vec<usize> = input.trim().split(' ').map(|x| x.parse().unwrap()).collect();

    for _ in 0..nm[0]+nm[1] {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).expect("read error");
        let name = str.trim().to_string();
        let count = ma.entry(name.clone()).or_insert(0);
        *count += 1;

        if *count > 1 {
            vt.push(name);
        }
    }

    vt.sort();

    println!("{}", vt.len());
    for name in vt {
        println!("{}", name);
    }
}