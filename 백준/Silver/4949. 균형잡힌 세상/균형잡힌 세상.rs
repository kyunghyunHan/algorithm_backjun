use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == ".\n" { // 입력 종료 조건
            break;
        }

        let mut stack = Vec::new();
        let mut is_valid = true;
        for c in input.chars() {
            if c == '(' || c == '[' {
                stack.push(c);
            } else if c == ')' {
                if let Some('(') = stack.pop() {
                    continue;
                } else {
                    is_valid = false;
                    break;
                }
            } else if c == ']' {
                if let Some('[') = stack.pop() {
                    continue;
                } else {
                    is_valid = false;
                    break;
                }
            }
        }

        if stack.is_empty() && is_valid {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
