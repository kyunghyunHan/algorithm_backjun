

use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let s = input.trim().chars().collect::<Vec<char>>();

    let mut dna: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    dna.insert("AA", "A");
    dna.insert("AG", "C");
    dna.insert("AC", "A");
    dna.insert("AT", "G");
    dna.insert("GA", "C");
    dna.insert("GG", "G");
    dna.insert("GC", "T");
    dna.insert("GT", "A");
    dna.insert("CA", "A");
    dna.insert("CG", "T");
    dna.insert("CC", "C");
    dna.insert("CT", "G");
    dna.insert("TA", "G");
    dna.insert("TG", "A");
    dna.insert("TC", "G");
    dna.insert("TT", "T");

    let mut s = s;

    loop {
        if s.len() == 1 {
            break;
        }
        let ss = vec![s[s.len() - 2], s[s.len() - 1]].iter().collect::<String>();
        if dna.contains_key(&ss.as_str()) {
            s.drain(s.len() - 2..);
            s.push(dna[&ss.as_str()].chars().next().unwrap());
        }
    }
    writeln!(writer,"{}", s.iter().collect::<String>()).unwrap();

    writer.flush().unwrap();

}