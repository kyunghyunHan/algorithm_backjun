use std::io::{BufReader,BufRead,BufWriter,stdin,Write,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
  let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        
        let mut zero = 1;
        let mut one = 0;

        for _ in 0..n {
            let temp = zero;
            zero = one;
            one = zero + temp;
        }

        writeln!(writer,"{} {}", zero, one).unwrap();
    }
    writer.flush().unwrap();
}
