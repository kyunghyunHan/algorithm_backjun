use std::io;
fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let n_arr: Vec<usize> = input_a
        .split_whitespace()
        .map(|x| -> usize { x.parse().unwrap() })
        .collect();
    let n = n_arr[0];
    let m = n_arr[1];
    let mut max_num = 0;
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).unwrap();
    let mut num_arr: Vec<usize> = input_b
        .split_whitespace()
        .map(|x| -> usize { x.parse().unwrap() })
        .collect();
    let mut total = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                total = num_arr[i] + num_arr[j] + num_arr[k];
                if total > m {
                    continue;
                }
                if max_num < total {
                    max_num = total;
                }
            }
        }
    }
    println!("{}", max_num)
}
