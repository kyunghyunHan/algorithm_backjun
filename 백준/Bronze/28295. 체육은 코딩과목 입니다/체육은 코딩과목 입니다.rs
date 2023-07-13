use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut x= 1080;
for i in 0..10{
    let mut input= String::new();

    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    match n {
        1 => x += 90,
        2 => x += 180,
        3 => x -= 90,
        _ => (),
    }
}
x /= 90;
    x %= 4;
if x==0{
    writeln!(writer,"{}","N").unwrap();
}else if x==1 {
    writeln!(writer,"{}","E").unwrap();

}else if x==2 {
    writeln!(writer,"{}","S").unwrap();

}else {
    writeln!(writer,"{}","W").unwrap();

}
}
