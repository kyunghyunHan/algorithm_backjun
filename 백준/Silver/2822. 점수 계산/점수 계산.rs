use std::io;

fn main() {
    let mut score = [0; 9];
    let mut sum = 0;

    for i in 0..8 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        score[i] = input.trim().parse().unwrap();
        sum += score[i];
    }

    score[8] = 150;
    let mut min = 8;

    for _ in 0..3 {
        for j in 0..8 {
            if score[j] != -1 && score[min] > score[j] {
                min = j;
            }
        }

        sum -= score[min];
        score[min] = -1;
        min = 8;
    }

    println!("{}", sum);

    for i in 0..8 {
        if score[i] != -1 {
            print!("{} ", i+1);
        }
    }
}
