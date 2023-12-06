use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let sequence: Vec<i32> = input
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect();
let mut position = 0;
for &num in sequence.iter() {
    position += num;
}
if position < 0 {
    writeln!(writer,"Left").unwrap();
} else if position > 0 {
    writeln!(writer,"Right").unwrap();
} else {
    writeln!(writer,"Stay").unwrap();
}

}