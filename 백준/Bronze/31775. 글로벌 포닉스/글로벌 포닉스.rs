use std::{
    cmp::min,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut count = 0;
    let mut l = false;
    let mut k = false;
    let mut p = false;

    let mut global_ponix = Vec::new();

    for i in 0..3 {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let st = &input.chars().collect::<Vec<char>>();
        global_ponix.push(input.trim().to_string()); // Collect input and trim whitespace

     

    }
    for i in global_ponix {
        let first_char = i.chars().next().unwrap(); // Get the first character
        match first_char {
            'l' => l = true,
            'k' => k = true,
            'p' => p = true,
            _ => {}
        }
    }
    if l && k && p {
        writeln!(writer,"{}","GLOBAL").unwrap();
     }else{
        writeln!(writer,"{}","PONIX").unwrap();

     }
    writer.flush().unwrap();
}
