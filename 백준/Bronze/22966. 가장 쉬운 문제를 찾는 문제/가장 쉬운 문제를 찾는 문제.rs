use std::io::{BufWriter, BufReader, BufRead, Write, stdin, stdout};
use std::collections::HashMap;

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let num: isize = input.trim().parse().unwrap();
    let mut map: HashMap<isize, String> = HashMap::new();
    for _ in 0..num {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut num = input.trim().split_whitespace();
        let a = num.next().unwrap().to_string();
        let b: isize = num.next().unwrap().parse().unwrap();
        map.insert(b, a);
    }

    // HashMap의 값을 기준으로 오름차순 정렬
    let mut sorted_values: Vec<(&isize, &String)> = map.iter().collect();
    sorted_values.sort_by(|a, b| a.0.cmp(b.0));

    // 최소값에 해당하는 값을 출력
    if let Some((_, value)) = sorted_values.first() {
        writeln!(writer, "{}", value).unwrap();
    }

    
}
