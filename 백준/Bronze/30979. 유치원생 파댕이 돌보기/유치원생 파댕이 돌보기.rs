use  std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let f:i32=  input.trim().split_whitespace().map(|x|x.parse::<i32>().unwrap()).sum();
    if f >=t{
        writeln!(writer,"{}","Padaeng_i Happy").unwrap();
    }else{
        writeln!(writer,"{}","Padaeng_i Cry").unwrap();
    }
    writer.flush().unwrap();
}