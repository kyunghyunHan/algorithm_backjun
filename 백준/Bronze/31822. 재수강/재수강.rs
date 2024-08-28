use std::{
    cmp::min,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let a = &input.trim().to_string()[..5];
    input.clear();
    reader.read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse::<i32>().unwrap();
    let mut count = 0;
    for i in 0..b {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let c= &input.trim().to_string()[..5];
        if a==c{
            count+=1;
        }
    }
    writeln!(writer,"{}",count).unwrap();
    writer.flush().unwrap();
}
