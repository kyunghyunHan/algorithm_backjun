use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    let n = match input.next() {
        Some(Ok(line)) => line.parse::<i32>().unwrap(),
        _ => {
            panic!("ERROR")
        }
    };

    let mut food_list = match input.next() {
        Some(Ok(line)) => line
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        _ => {
            panic!("ERROR")
        }
    };
    let food_list2 = match input.next() {
        Some(Ok(line)) => line
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        _ => {
            panic!("ERROR")
        }
    };
    for i in food_list2 {
        food_list.retain(|item| *item != i);
    }

    for i in food_list {
        writeln!(writer, "{} ", i).unwrap();
    }
    writer.flush().unwrap();
}
