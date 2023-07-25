use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let p: i32 = input_line.trim().parse().unwrap();

    let mut software = 0;
    let mut embedded = 0;
    let mut ai = 0;
    let mut no_dept = 0;

    for _ in 0..p {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let grade: i32 = iter.next().unwrap().parse().unwrap();
        let class_num: i32 = iter.next().unwrap().parse().unwrap();
        let _num: i32 = iter.next().unwrap().parse().unwrap();

        if grade == 1 {
            no_dept += 1;
        } else if class_num <= 2 {
            software += 1;
        } else if class_num == 3 {
            embedded += 1;
        } else if class_num == 4 {
            ai += 1;
        }
    }

    println!("{}\n{}\n{}\n{}", software, embedded, ai, no_dept);
}
