use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};


fn main(){

    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse::<usize>().unwrap();
let mut writer= BufWriter::new(stdout().lock());
    for i in 0..n{

        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
         
        let n= nums.next().unwrap();
        let m =nums.next().unwrap();

        writeln!(writer,"{} {}",(m*2)-n,n-m).unwrap();
        
        }
}