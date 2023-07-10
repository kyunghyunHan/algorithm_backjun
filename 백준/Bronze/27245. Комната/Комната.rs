use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());

let mut v:Vec<usize>= vec![];

for i in 0..3 {
    let mut input= String::new();

    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse().unwrap();
    v.push(n);
}
if usize::min(v[0], v[1])/v[2]>=2 && usize::max(v[0], v[1])/usize::min(v[0], v[1])<=2{
writeln!(writer,"good").unwrap();
}else {
    writeln!(writer,"bad").unwrap();

}
}
