use std::io::{BufWriter, BufRead, BufReader, Write, stdin, stdout};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    // Read n from input
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut num = input.trim().split_whitespace().map(|x| x.parse::<isize>().unwrap());
    let a= num.next().unwrap();
    let b= num.next().unwrap();
    let c= num.next().unwrap();
    let d= num.next().unwrap();


    writeln!(writer,"{}",(56*a)+(24*b)+(14*c)+(6*d)).unwrap();
}
