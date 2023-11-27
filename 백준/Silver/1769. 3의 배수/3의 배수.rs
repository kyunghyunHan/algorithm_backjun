use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();

    let mut n= input.trim().to_string();
    let mut t= 0;
    let mut c=0;
    while n.len()>=2{
        t = n.chars().into_iter().map(|x|x.to_string().parse().unwrap()).collect::<Vec<u64>>().iter().sum();
        n= t.to_string();
        c+=1;
    }

    writeln!(writer,"{}",c).unwrap();
    if n.parse::<u64>().unwrap() % 3==0{
        writeln!(writer,"{}","YES").unwrap();
    }else{
        writeln!(writer,"{}","NO").unwrap();

    }

    writer.flush().unwrap();
}