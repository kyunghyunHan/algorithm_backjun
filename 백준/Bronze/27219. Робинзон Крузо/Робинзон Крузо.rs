use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();


    reader.read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().expect("Invalid input.");

 for i  in 0..n/5{
 write!(writer,"{}","V").unwrap();
} 

for i in 0..n%5{
    write!(writer,"{}","I").unwrap();

}
}