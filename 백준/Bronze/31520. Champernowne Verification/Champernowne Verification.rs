use std::{
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
    result::Result,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(stdin());
    let mut writer = BufWriter::new(stdout());

    let mut input = String::new();
    reader.read_line(&mut input)?;

    let n: Vec<String> = input.trim().chars().map(|x| x.to_string()).collect();

    for i in 0..n.len() {
        if n[i].parse::<i32>()? != i as i32 + 1 {
            writeln!(writer, "{}", -1)?;
            writer.flush()?;
            return Ok(());
        }
    }

    writeln!(writer, "{}", n[n.len() - 1])?;
    writer.flush()?;
    
    Ok(())
}
