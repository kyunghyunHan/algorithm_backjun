use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

use std::collections::HashMap;
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();
    let mut hash_map: HashMap<i32, bool> = HashMap::new();
    for i in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let x = input.trim().to_string();

        if x.len() > 7 {
            continue;
        }
        hash_map.insert(x.parse::<i32>().unwrap(), true);
    }
    let mut i = 0;
    loop {
        match hash_map.get(&i) {
            Some(false) => {
                writeln!(writer, "{}", i).unwrap();
                writer.flush().unwrap();
                break;
            }
            Some(true) => {
                // 이미 true로 설정된 경우이므로, i를 증가시키고 계속 탐색
                i += 1;
            }
            None => {
                // i가 HashMap에 존재하지 않는 경우, 출력하고 종료
                writeln!(writer, "{}", i).unwrap();
                writer.flush().unwrap();
                break;
            }
        }
    }
}
