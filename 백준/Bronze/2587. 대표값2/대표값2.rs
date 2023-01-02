use std::io;

fn main() {
    let mut arr: Vec<i32> = vec![];
    let mut sum = 0;
    for i in 0..5 {
        let mut input_d = String::new();
        io::stdin().read_line(&mut input_d).unwrap();
        let v: Vec<i32> = input_d
            .split_whitespace()
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect();

        arr.push(v[0]);
        sum += v[0];
        arr.sort();
    }

    println!("{}\n{}", sum / 5, arr[2])
}
