
use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
use std::collections::BTreeMap; // HashMap 대신 BTreeMap 사용

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut arr = BTreeMap::new(); // HashMap 대신 BTreeMap 사용

    for (i, line) in reader.lines().take(n).enumerate() {
        let buffer = line.unwrap();
        let mut nums = buffer.trim().split_whitespace();
        let name = nums.next().unwrap().to_string();
        let status = nums.next().unwrap().to_string();

        if status == "enter" {
            arr.insert(name, ()); // 이름만 저장, 값은 ()
        } else {
            arr.remove(&name); // 나갔으면 제거
        }
    }

    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    // BTreeMap의 keys 메서드를 사용하여 키를 오름차순으로 순회하면서 출력
    for key in arr.keys().rev() {
        writeln!(writer, "{}", key).unwrap();
    }
}