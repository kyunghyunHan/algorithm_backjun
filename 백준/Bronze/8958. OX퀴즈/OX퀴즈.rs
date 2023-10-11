use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input =  String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    for i in 0..n{
         input.clear();
         reader.read_line(&mut input).unwrap();
         let mut count= 0;
         let mut result= 0;
         let s= input.trim();
         for i in s.chars(){
            if i=='O'{
                count+=1;  
                result+=count;
            }else if i== 'X'{
                count= 0;
            }
         }

         writeln!(writer,"{}",result).unwrap();
    }
    writer.flush().unwrap();
}