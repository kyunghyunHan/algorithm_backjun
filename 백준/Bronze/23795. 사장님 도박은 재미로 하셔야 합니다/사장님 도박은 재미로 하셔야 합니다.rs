use std::io::{BufWriter,BufReader,BufRead,Write,stdin,stdout};

fn main(){


    let mut reader= BufReader::new(stdin().lock());
   let mut writer= BufWriter::new(stdout().lock());
    let mut count=0;
    loop{  
        let mut input= String::new();
        reader.read_line(&mut input).unwrap();
        let mut  num= input.trim().split_whitespace().map(|x|x.parse::<isize>().unwrap());
        let pay: isize= num.next().unwrap();
        if pay ==-1 {
            break;
        }
        count+=pay;
       
       
    }
   writeln!(writer,"{}",count).unwrap();
} 