use std::io::{stdin,stdout,BufReader,BufRead,BufWriter,Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut counter = 0;
 
    for _ in 0..6 {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let impit= input.trim();

    
        if impit == "W" {
            counter += 1;
          
        }
    }

    if counter ==5 || counter==6{
        writeln!(writer, "{}", "1").unwrap();
    } else if counter ==3 || counter==4{
        writeln!(writer, "{}", "2").unwrap();
    }else if counter ==2 || counter==1{
        writeln!(writer, "{}", "3").unwrap();
    }
    else {
        writeln!(writer, "{}", "-1").unwrap();
    }
}
