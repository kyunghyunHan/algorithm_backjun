use std::collections::BinaryHeap;
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main() {
     let mut reader= BufReader::new(stdin().lock());
     let mut writer= BufWriter::new(stdout().lock());
     

    let  mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse().unwrap();
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();

    for _ in 0..n {
input.clear();
reader.read_line(&mut input).unwrap();
let x:i32= input.trim().parse().unwrap();       
 if x == 0 {
            if let Some(val) = heap.pop() {
                writeln!(writer,"{}", val.abs()).unwrap();
            } else {
                writeln!(writer,"0").unwrap();
            }
        } else {
            heap.push(-x);
        }
    }
}
