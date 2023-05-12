
use std::collections::BinaryHeap;
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main() {
    let mut reader= BufReader::new(stdin().lock());

    
    let mut input = String::new();
    
    let mut result: Vec<i32> = Vec::new();
let mut writer=BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Failed to parse input");
    
    let mut q: BinaryHeap<i32> = BinaryHeap::new();
    for _ in 0..n {
        input.clear();
        
        reader.read_line(&mut input).unwrap();
        let x: i32 = input.trim().parse().expect("Failed to parse input");
        
        if x != 0 {
            q.push(x);
        } else {
            if let Some(max) = q.pop() {
                result.push(max);
            } else {
                result.push(0);
            }
        }
    }
    
    for i in 0..result.len() {
        writeln!(writer,"{}", result[i]).unwrap();
    }

}
