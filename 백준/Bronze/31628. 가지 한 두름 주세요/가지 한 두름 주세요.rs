use std::io;

fn input(str: &mut Vec<Vec<String>>) {
    let mut input_string = String::new();
    for i in 0..10 {
        input_string.clear();
        io::stdin().read_line(&mut input_string).expect("Failed to read line");
        let values: Vec<String> = input_string.split_whitespace().map(|s| s.to_string()).collect();
        str.push(values);
    }
}

fn solve(str: &Vec<Vec<String>>) -> i32 {
    for i in 0..10 {
        let mut is_find = true;
        let first = &str[i][0];
        for j in 0..10 {
            if first != &str[i][j] {
                is_find = false;
            }
        }
        if is_find {
            return 1;
        }

        is_find = true;
        let first = &str[0][i];
        for j in 0..10 {
            if first != &str[j][i] {
                is_find = false;
            }
        }
        if is_find {
            return 1;
        }
    }
    return 0;
}

fn main() {
    let mut str: Vec<Vec<String>> = Vec::new();
    input(&mut str);
    let result = solve(&str);
    println!("{}", result);
}
