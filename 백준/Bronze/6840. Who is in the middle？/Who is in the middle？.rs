use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){

    let mut v= [0;3];

    let mut reader = BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());

    for i in 0..3{
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let n= input.trim().parse::<usize>().unwrap();
        v[i]= n;
    }

    v.sort();

    writeln!(writer,"{}",v[1]).unwrap();
}
