use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let a = line.parse::<usize>().unwrap();
        if let Some(Ok(line)) = input.next() {
            let b = line.parse::<usize>().unwrap();
            let result = (a + b) % 12;
             if result ==0{
                writeln!(writer,"{}",12).unwrap();
             }else{
                writeln!(writer,"{}",result).unwrap();

             }
        }
    }
}
