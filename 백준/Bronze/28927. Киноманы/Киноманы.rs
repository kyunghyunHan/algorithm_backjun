use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let max_stats: Vec<i32> = input
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mel_stats: Vec<i32> = input
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect(); 
let max_time = max_stats[0] * 3 + max_stats[1] * 20 + max_stats[2] * 120;
let mel_time = mel_stats[0] * 3 + mel_stats[1] * 20 + mel_stats[2] * 120;
if max_time > mel_time {
    writeln!(writer,"Max").unwrap();
} else if mel_time > max_time {
    writeln!(writer,"Mel").unwrap();
} else {
    writeln!(writer,"Draw").unwrap();
}
writer.flush().unwrap();
}