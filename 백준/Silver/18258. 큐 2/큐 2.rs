use std::collections::VecDeque;
use std::io::{self, BufRead,Write,BufReader,BufWriter,stdin,stdout};

fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse().unwrap();
    let mut writer= BufWriter::new(stdout().lock());
    let mut queue: VecDeque<i32> = VecDeque::new();
    for _ in 0..n {
        buffer.clear();
        reader.read_line(&mut buffer).unwrap();
        let mut iter = buffer.trim().split_whitespace();
        let command = iter.next().unwrap();
        match command {
            "push" => {
                let num = iter.next().unwrap().parse().unwrap();
                queue.push_back(num);
            }
            "pop" => {
                if let Some(num) = queue.pop_front() {
                    writeln!(writer,"{}", num).unwrap();
                } else {
                    writeln!(writer,"-1").unwrap();
                }
            }
            "size" => {
                writeln!(writer,"{}", queue.len()).unwrap();
            }
            "empty" => {
                writeln!(writer,"{}", queue.is_empty() as i32).unwrap();
            }
            "front" => {
                if let Some(num) = queue.front() {
                    writeln!(writer,"{}", num).unwrap();
                } else {
                    writeln!(writer,"-1").unwrap();
                }
            }
            "back" => {
                if let Some(num) = queue.back() {
                    writeln!(writer,"{}", num).unwrap();
                } else {
                    writeln!(writer,"-1").unwrap();
                }
            }
            _ => unreachable!(),
        }
    }
}
