use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut jeminis = [0; 10];
    let mut startlink = [0; 10];
    let mut stat = 0;
    let mut cur_j_score = 0;
    let mut cur_s_score = 0;

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    for (i, num) in input.split_whitespace().map(|x| x.parse().unwrap()).enumerate() {
        jeminis[i + 1] = num;
    }

    input.clear();
    reader.read_line(&mut input).unwrap();
    for (i, num) in input.split_whitespace().map(|x| x.parse().unwrap()).enumerate() {
        startlink[i + 1] = num;
    }

    for i in 1..=9 {
        cur_j_score += jeminis[i];
        if cur_j_score > cur_s_score {
            stat = 1;
        }
        cur_s_score += startlink[i];
        if cur_j_score < cur_s_score && stat == 1 {
            stat = 2;
        }
    }

    if stat == 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
