use std::io;

fn main() {
    let mut test = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30,
    ];
    for i in 0..28 {
        let mut numbers_arry = String::new();
        io::stdin().read_line(&mut numbers_arry).unwrap();
        let numbers: Vec<&str> = numbers_arry.split_whitespace().collect();
        let number_first = numbers[0].parse::<i32>().unwrap();
        // for j in test {
        //     if j == number_first {
        //         test.remove(j);
        //     }
        // }
        test.retain(|value| *value != number_first);
    }
    for i in test {
        println!("{}", i);
    }
    // let mut some_vec = vec![0, 10, 20, 30];
    // some_vec.retain(|value| *value != 10);
}
