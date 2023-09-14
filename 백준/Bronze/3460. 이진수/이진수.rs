use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();

    let mut n = input.trim().parse::<i32>().unwrap();

    
    for i in 0..n{
         input.clear();
         reader.read_line(&mut input).unwrap();
         let  n = input.trim().parse::<i32>().unwrap();
         let binary_string = format!("{:b}", n);
         let mut binary_arr: Vec<char>= binary_string.chars().collect::<Vec<char>>();
         binary_arr.reverse();
         for i in 0..binary_arr.len(){
            if binary_arr[i]=='1'{
                write!(writer,"{} ",i).unwrap();
            }
         }
         writeln!(writer).unwrap();
        }
}