use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut writer= BufWriter::new(stdout().lock());
    let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let mut one= 0;
    let mut two = 0;
    for i in nums{
        if i ==1{
            one+=1;
        }else if i ==2 {
            two+=1;
        }
    }
    
    if one>two{
        writeln!(writer,"{}",1).unwrap();
    }else{
        writeln!(writer,"{}",2).unwrap();

    }
}