use std::io;

fn main() {
    let mut arr: [i32; 9] = [0; 9];
    let mut sum = 0;

    for i in 0..9 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let num: i32 = input.trim().parse().unwrap();
        arr[i] = num;
        sum += num;
    }

    arr.sort();

    for i in 0..8 {
        for j in (i + 1)..9 {
            if sum - arr[i] - arr[j] == 100 {
                for k in 0..9 {
                    if k == i || k == j {
                        continue;
                    }
                    println!("{}", arr[k]);
                }
                return;
            }
        }
    }
}
