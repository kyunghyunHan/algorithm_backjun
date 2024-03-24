use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main()-> std::io::Result<()>{
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input =String::new();
    reader.read_line(&mut input).unwrap();
    let r = input.trim().parse::<f64>().unwrap();
    let area = r * r * std::f64::consts::PI;
    let circumference = 2.0 * r * r;

    println!("{:.6}", area);
    println!("{:.6}", circumference);
    Ok(())
}
