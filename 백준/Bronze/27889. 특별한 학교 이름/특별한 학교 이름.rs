use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut input = String::new();

reader.read_line(&mut input).unwrap();
let mut writer= BufWriter::new(stdout().lock());

let s= input.trim();


if s=="NLCS"{
    writeln!(writer,"{}","North London Collegiate School").unwrap();
}else if s=="BHA" {
    writeln!(writer,"{}","Branksome Hall Asia").unwrap();

}else if s=="KIS"{
    writeln!(writer,"{}","Korea International School").unwrap();

}else if s=="SJA"{
    writeln!(writer,"{}","St. Johnsbury Academy").unwrap();

}

}