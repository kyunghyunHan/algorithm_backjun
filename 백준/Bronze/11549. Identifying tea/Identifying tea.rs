use std::io::{stdin,stdout,BufReader,BufRead,BufWriter,Write};

fn main(){
    let mut reader = BufReader::new(stdin().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let a: usize = input.trim().parse().unwrap();
    let mut counter= 0;
    let mut input2= String::new();
    reader.read_line(&mut input2).unwrap();
    let mut nums= input2.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
    
    for i in 0..5{
        let mut n= nums.next().unwrap();
        if  n==a{
            counter+=1
        }
    }


    let mut writer= BufWriter::new(stdout().lock());
    writeln!(writer,"{}",counter).unwrap();
}