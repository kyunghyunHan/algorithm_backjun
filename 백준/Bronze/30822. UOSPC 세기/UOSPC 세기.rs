use std::io::{BufReader, BufRead, BufWriter, Write, stdin, stdout};
use std::collections::HashMap;

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();
    let s = input.trim().chars().collect::<Vec<char>>();
    let mut hm: HashMap<char, u32> = HashMap::new();
    hm.insert('u', 0);
    hm.insert('o', 0);
    hm.insert('s', 0);
    hm.insert('p', 0);
    hm.insert('c', 0);

    for i in &s {
        if let Some(count) = hm.get_mut(i) {
            // 해당 키가 존재하는 경우
            *count += 1; // 값에 1을 더함
        } 
    } 
    let mut count = 0;
    for i in 0..1000{
      if hm.values().all(|&x| x > i){
        count+=1;
      }
    }
    writeln!(writer,"{}",count).unwrap();
 
}
