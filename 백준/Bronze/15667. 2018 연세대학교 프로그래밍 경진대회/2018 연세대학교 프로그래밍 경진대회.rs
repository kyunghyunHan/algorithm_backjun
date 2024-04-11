use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().expect("Please enter a valid integer");

    let mut a = 0;
    let  n = n - 1;

    loop {
        a += 1;
        if a * a + a == n {
            break;
        }
    }

    writeln!(writer,"{}", a).unwrap();
    writer.flush().unwrap();
}