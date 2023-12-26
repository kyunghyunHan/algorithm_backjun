use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut inputs= input.trim().split_whitespace();
    let n:i32= inputs.next().unwrap().parse().unwrap();
    let lv= inputs.next().unwrap();
    
    if lv=="miss"{
        writeln!(writer,"{}",0).unwrap();
    }else if lv =="bad"{
        writeln!(writer,"{}",200 *n).unwrap();

    }else if lv =="cool"{
        writeln!(writer,"{}",400 *n).unwrap();

    }else if lv =="great"{
        writeln!(writer,"{}",600 *n).unwrap();

    }else if lv =="perfect"{
        writeln!(writer,"{}",1000 *n).unwrap();

    }
   writer.flush().unwrap();
}
