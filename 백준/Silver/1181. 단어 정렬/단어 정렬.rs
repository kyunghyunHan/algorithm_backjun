use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
use std::cmp::Ordering;
fn main(){
 
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input = String::new();
    let mut word: Vec<String> = vec![];
    reader.read_line(&mut input).unwrap();
    let n:i32=input.trim().parse().unwrap();

       // word 배열 입력받기
    for _ in 0..n {
       input.clear();
       reader.read_line(&mut input).unwrap();
        word.push(input.trim().to_string());
    }

    word.sort_by(cmp);
    let mut prev_word = "";

    for w in word.iter() {
        if w != prev_word {
        writeln!(writer,"{}", w).unwrap();
        prev_word = w;
        }
    }

     writer.flush().unwrap();
    
}

fn cmp(a: &String, b: &String) -> Ordering {
    // 1. 길이가 같다면, 사전순으로
    if a.len() == b.len() {
        return a.cmp(b);
    }
    // 2. 길이가 다르다면, 짧은 순으로
    else {
        return a.len().cmp(&b.len());
    }
}