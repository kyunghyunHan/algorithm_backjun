use std::io::{stdin,stdout,BufReader,BufRead,BufWriter,Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

 
    let mut _apple= 0;
    for i in (1..=3).rev()  {
    let mut input: String = String::new();
    reader.read_line(&mut input).unwrap();
    let impit:i128= input.trim().parse().unwrap();
    _apple+=impit*i
    }
    
    let mut _banana= 0;
    for i in (1..4).rev() {
    let mut input: String = String::new();
    reader.read_line(&mut input).unwrap();
    let impit:i128= input.trim().parse().unwrap();
    _banana+=impit*i
    }

  
    if _apple>_banana{
        writeln!(writer,"{}","A").unwrap();
    }else if _apple<_banana{
        writeln!(writer,"{}","B").unwrap();
    }else {
        writeln!(writer,"{}","T").unwrap();
    }
}
