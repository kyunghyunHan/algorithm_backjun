use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());
    let mut  input= String::new();
    reader.read_line(&mut input).unwrap();
    let n:usize= input.trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();
    let vec: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut copy_vec = vec.clone();
    copy_vec.sort_unstable();
    copy_vec.dedup();
    for i in vec {
        let idx = match copy_vec.binary_search(&i) {
            Ok(x) => x,
            Err(x) => x,
        };
        write!(writer,"{} ", idx).unwrap();

    }
    writer.flush().unwrap();
}