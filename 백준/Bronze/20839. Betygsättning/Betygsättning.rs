use std::io;

fn get_grade(a: f64, c: f64, e: f64, student: &[f64; 3]) -> char {
    if student[0] >= a && student[1] >= c && student[2] >= e {
        return 'A';
    }
    if student[0] >= a / 2.0 && student[1] >= c && student[2] >= e {
        return 'B';
    }
    if student[1] >= c && student[2] >= e {
        return 'C';
    }
    if student[1] >= c / 2.0 && student[2] >= e / 2.0 {
        return 'D';
    }
    'E'
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let a: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let c: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let e: f64 = iter.next().unwrap().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    iter = input.split_whitespace();
    let mut student: [f64; 3] = [0.0; 3];
    for i in 0..3 {
        student[i] = iter.next().unwrap().parse().expect("Invalid input");
    }

    let grade = get_grade(a, c, e, &student);
    println!("{}", grade);
}
