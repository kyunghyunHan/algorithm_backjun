use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();

    let height: f32 = input.trim().parse().unwrap();

    let bmi = weight / (height * height);

    // Determine weight category
    if bmi > 25.0 {
        writeln!(writer, "Overweight").unwrap();
    } else if bmi > 18.5 {
        writeln!(writer, "Normal weight").unwrap();
    } else {
        writeln!(writer, "Underweight").unwrap();
    }
}
