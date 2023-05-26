use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let n: usize = input.next().unwrap().unwrap().parse().unwrap();

    let mut count = 1;
    let mut makes_sense = true;

    for word in input.next().unwrap().unwrap().split_whitespace() {
        if word != "mumble" {
            let num: usize = word.parse().unwrap();
            if num != count {
                makes_sense = false;
                break;
            }
        }
        count += 1;
    }

    if makes_sense {
        println!("makes sense");
    } else {
        println!("something is fishy");
    }
}
