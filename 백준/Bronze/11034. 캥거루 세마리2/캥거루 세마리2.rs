use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout };

fn main() {
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  
    loop {
        let mut input_line = String::new();
        
        match reader.read_line(&mut input_line) {
            Ok(_) => {
                let mut split_iter = input_line.split_whitespace();
                let a: i32 = match split_iter.next() {
                    Some(val) => val.parse().unwrap(),
                    None => break, 
                };
                let b: i32 = match split_iter.next() {
                    Some(val) => val.parse().unwrap(),
                    None => break, 
                };
                let c: i32 = match split_iter.next() {
                    Some(val) => val.parse().unwrap(),
                    None => break, 
                };


                let result = (b - a).max(c - b) - 1;
                writeln!(writer,"{}", result).unwrap();
            }
            Err(_) => break, 
        }
    }
}
