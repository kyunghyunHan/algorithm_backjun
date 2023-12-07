use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main() {
  let mut reader= BufReader::new(stdin().lock());
  let mut writer= BufWriter::new(stdout().lock());
  let mut input = String::new();

   reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    input.clear();

    reader.read_line(&mut input).unwrap();
    let door_numbers: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut even_count = 0;
    let mut odd_count = 0;

    for &num in &door_numbers {
        if num % 2 == 0 {
            even_count += 1;
        } else {
            odd_count += 1;
        }
    }

    if even_count > odd_count {
        writeln!(writer,"Happy").unwrap();
    } else {
        writeln!(writer,"Sad").unwrap();
    }
}
