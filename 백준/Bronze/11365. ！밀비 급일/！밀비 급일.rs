use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());

    loop{
        let mut input= String::new();
        reader.read_line(&mut input).unwrap();
        let s= input.trim();

        if s=="END"{
            break;
        }

        for i in s.chars().rev(){
            write!(writer,"{}",i).unwrap();
        }
        writeln!(writer,"").unwrap();
    }
}