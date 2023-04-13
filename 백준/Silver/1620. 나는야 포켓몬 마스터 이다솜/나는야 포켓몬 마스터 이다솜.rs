use std::collections::HashMap;
use std::io::{stdin, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let mut nums = buffer.trim().split_whitespace();
    let n = nums.next().unwrap().parse::<usize>().unwrap();
    let m = nums.next().unwrap().parse::<usize>().unwrap();

    let mut name_to_number: HashMap<String, usize> = HashMap::new();
    let mut number_to_name: HashMap<usize, String> = HashMap::new();

    for i in 0..n {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        let name = buffer.trim().to_string();
        name_to_number.insert(name.clone(), i + 1);
        number_to_name.insert(i + 1, name);
    }

    let mut results = Vec::new();

    for _ in 0..m {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        let query = buffer.trim().to_string();

        if let Ok(k) = query.parse::<usize>() {
            if let Some(name) = number_to_name.get(&k) {
                results.push(name.to_string());
            }
        } else if let Some(&k) = name_to_number.get(&query) {
            results.push(k.to_string());
        }
    }

    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{}", results.join("\n")).unwrap();
}