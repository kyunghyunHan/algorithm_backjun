use std::collections::HashMap;
use std::vec::Vec;
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut ma: HashMap<String, i32> = HashMap::new();
    let mut vt: Vec<String> = Vec::new();

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let nm: Vec<usize> = input.trim().split(' ').map(|x| x.parse().unwrap()).collect();

    for _ in 0..nm[0]+nm[1] {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let name = input.trim().to_string();
        let count = ma.entry(name.clone()).or_insert(0);
        *count += 1;

        if *count > 1 {
            vt.push(name);
        }
    }

    vt.sort();

    writeln!(writer,"{}", vt.len()).unwrap();
    for name in vt {
        writeln!(writer,"{}", name).unwrap();
    }
}