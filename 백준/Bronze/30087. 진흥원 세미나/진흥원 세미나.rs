use std::collections::HashMap;
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    let mut seminar_map = HashMap::new();
    seminar_map.insert("Algorithm", "204");
    seminar_map.insert("DataAnalysis", "207");
    seminar_map.insert("ArtificialIntelligence", "302");
    seminar_map.insert("CyberSecurity", "B101");
    seminar_map.insert("Network", "303");
    seminar_map.insert("Startup", "501");
    seminar_map.insert("TestStrategy", "105");

    for _ in 0..n {
input.clear();
reader.read_line(&mut input).unwrap();
let seminar= input.trim();
        if let Some(&room) = seminar_map.get(&seminar[..]) {
            writeln!(writer,"{}", room).unwrap();
        }
    }
}
