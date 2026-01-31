use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
    
    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        if n <= 10000{
            println!("Accepted");
        }else{
            println!("Time limit exceeded");
        }
    }
    writer.flush().unwrap();
}