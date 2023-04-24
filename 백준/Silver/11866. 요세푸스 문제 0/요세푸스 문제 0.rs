use std::collections::VecDeque;

use std::io::{stdin, BufRead, BufReader, BufWriter, Write,stdout};


fn main() {
    let mut people: VecDeque<usize> = VecDeque::new();
    let mut removed_people_num = 0;
    let mut current = 1;

    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let mut nums = buffer.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());

    let n = nums.next().unwrap();
    let k = nums.next().unwrap();

    for i in 1..=n {
        people.push_back(i);
    }

    print!("<");

    while let Some(front) = people.pop_front() {
        if current % k == 0 {
            print!("{}", front);
            removed_people_num += 1;

            if removed_people_num != n {
                print!(", ");
            }
        } else {
            people.push_back(front);
        }

        current += 1;
    }

    println!(">");
    stdout().flush().expect("Failed to flush stdout");
}
