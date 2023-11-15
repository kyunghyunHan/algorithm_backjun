use std::collections::VecDeque;
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    /*
     양방향 큐 deque
     양방향에서 element를 추거하거나 제거할수있음
     push와 pop이 압도적으로빠름
     */
    let mut input = String::new();
    let mut dq: VecDeque<i32> = VecDeque::new();

    reader.read_line(&mut input).unwrap();

    let n= input.trim().parse().unwrap();

    for _ in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts[0] {
            "push_front" => {
                let x: i32 = parts[1].trim().parse().expect("Invalid input");
                dq.push_front(x);
            }
            "push_back" => {
                let x: i32 = parts[1].trim().parse().expect("Invalid input");
                dq.push_back(x);
            }
            "pop_front" => {
                if let Some(x) = dq.pop_front() {
                    writeln!(writer,"{}", x).unwrap();
                } else {
                    writeln!(writer,"{}", -1).unwrap();
                }
            }
            "pop_back" => {
                if let Some(x) = dq.pop_back() {
                    writeln!(writer,"{}", x).unwrap();
                } else {
                    writeln!(writer,"{}", -1).unwrap();
                }
            }
            "size" => {
                writeln!(writer,"{}", dq.len()).unwrap();
            }
            "empty" => {
                writeln!(writer,"{}", dq.is_empty() as i32).unwrap();
            }
            "front" => {
                if let Some(&x) = dq.front() {
                    writeln!(writer,"{}", x).unwrap();
                } else {
                    writeln!(writer,"{}", -1).unwrap();
                }
            }
            "back" => {
                if let Some(&x) = dq.back() {
                    writeln!(writer,"{}", x).unwrap();
                } else {
                    writeln!(writer,"{}", -1).unwrap();
                }
            }
            _ => {
                writeln!(writer,"{}", "Invalid command").unwrap();
            }
        }
    }
    writer.flush().unwrap();
}