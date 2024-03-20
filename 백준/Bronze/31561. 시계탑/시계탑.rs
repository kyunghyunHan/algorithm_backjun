use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse::<f32>().unwrap();
    let ans = if n > 30. {
        15. + (n - 30.) * 3. / 2.
    } else {
        n / 2.
    };

    writeln!(writer,"{:.1}",ans).unwrap();
    writer.flush().unwrap();
    
}
