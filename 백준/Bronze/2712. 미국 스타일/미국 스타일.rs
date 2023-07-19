use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input =String::new();
    let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();
    for i in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut s:Vec<&str>= input.trim().split_whitespace().collect();
        if s[1]=="kg"{
            writeln!(writer,"{:.4} {}",s[0].parse::<f64>().unwrap()*2.2046,"lb").unwrap();

        }else if s[1]=="lb"{
            writeln!(writer,"{:.4} {}",s[0].parse::<f64>().unwrap()*0.4536,"kg").unwrap();

        }
        else if s[1]=="l"{
            writeln!(writer,"{:.4} {}",s[0].parse::<f64>().unwrap()*0.2642,"g").unwrap();

        }else if s[1]=="g"{
            writeln!(writer,"{:.4} {}",s[0].parse::<f64>().unwrap()*3.7854,"l").unwrap();

        }
    }
}