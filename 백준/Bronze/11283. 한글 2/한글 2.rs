use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main()-> std::io::Result<()>{
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input =String::new();
    reader.read_line(&mut input).unwrap();
    let char_code= input.trim().chars().next().unwrap() as u32;
    let unicode_offset = 44031;
    let result = char_code - unicode_offset;

    writeln!(writer,"{}",result)?;
    Ok(())
}