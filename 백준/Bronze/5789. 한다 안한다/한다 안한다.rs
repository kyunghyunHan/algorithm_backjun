use std::io::{BufRead,BufReader,Write,stdin,stdout,BufWriter};


fn main(){
    let mut reader =BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());

    reader.read_line(&mut input).unwrap();

    let n  = input.trim().parse().unwrap();

    for i in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let n: Vec<char> = input.trim().chars().collect();
        let k = n.len() / 2 - 1;
        if n[k] == n[n.len() - k - 1] {
            println!("Do-it");
        } else {
            println!("Do-it-Not");
        }
    }

}