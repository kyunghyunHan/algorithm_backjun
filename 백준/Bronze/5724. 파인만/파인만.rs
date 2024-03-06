use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();

    loop{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
         
        if n==0{
            break;
        }else{
            let mut cnt= 0;
            for i in 0..n{
                 cnt+=(n-i).pow(2)
            }
            writeln!(writer,"{}",cnt).unwrap();

        }
    } 
    writer.flush().unwrap();
}
