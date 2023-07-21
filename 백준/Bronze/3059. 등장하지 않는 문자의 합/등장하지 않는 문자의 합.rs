use std::io;

fn main() {
    let mut test = String::new();
    io::stdin().read_line(&mut test).expect("Failed to read line");
    let test: i32 = test.trim().parse().expect("Invalid input");

    for _ in 0..test {
        let mut str = String::new();
        io::stdin().read_line(&mut str).expect("Failed to read line");
        let str = str.trim();

        let len = str.len();

        let mut arr = [0; 26];
        let mut cnt: i32;

        for i in 0..len {
            cnt = (str.chars().nth(i).unwrap() as i32) - 65; // A = 65 Z = 90
            arr[cnt as usize] += 1; // characters used in the string = 1, unused characters = 0
        }

        let mut sum = 0;

        for i in 0..26 {
            if arr[i] == 0 {
                sum += i as i32 + 65; // add the ASCII values of unused characters
            }
        }

        println!("{}", sum);
    }
}
