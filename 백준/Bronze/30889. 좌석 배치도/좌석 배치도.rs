use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut n= input.trim().parse().unwrap();
    let mut arr: Vec<String> = Vec::with_capacity(n);
    
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut s= input.trim().to_string();
        arr.push(s);
    }
    let mut movie = vec![vec!['.'; 20]; 10];
    for chair in &arr {
        let x = (chair.chars().nth(0).unwrap() as u8 - b'A') as usize;
        let y = chair[1..].parse::<usize>().unwrap() - 1;
        
        movie[x][y] = 'o';
    }
    for row in movie {
        writeln!(writer,"{}", row.into_iter().collect::<String>()).unwrap();
    
    }
    writer.flush().unwrap();
}