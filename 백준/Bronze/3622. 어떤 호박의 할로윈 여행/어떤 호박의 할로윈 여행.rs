use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input   = String::new();
    reader.read_line(&mut input).unwrap();
    let mut inputs= input.trim().split_ascii_whitespace().map(|x|x.parse::<i32>().unwrap());

    let A= inputs.next().unwrap();
    let a= inputs.next().unwrap();
    let B= inputs.next().unwrap();
    let b= inputs.next().unwrap();
    let p= inputs.next().unwrap();
    let mut c= false;
    if A+B <=p {
        c= true;
    }else if a >=B &&p>=A{
        c= true;
    }else if A<=b && p>=B {
        c= true
    }else {
        c= false
    }
 
    match c {
        true=>writeln!(writer,"{}","Yes").unwrap(),
        false=>writeln!(writer,"{}","No").unwrap()
    }
}