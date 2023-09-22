use std::{io::{BufReader,BufRead,BufWriter,Write,stdin,stdout},collections::HashMap};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse().unwrap();     
    let mut diction:HashMap<&str,f64> = HashMap::new();
    diction.insert("A+", 4.3);
    diction.insert("A0", 4.0);
    diction.insert("A-", 3.7);
    diction.insert("B+", 3.3);
    diction.insert("B0", 3.0);
    diction.insert("B-", 2.7);
    diction.insert("C+", 2.3);
    diction.insert("C0", 2.0);
    diction.insert("C-", 1.7);
    diction.insert("D+", 1.3);
    diction.insert("D0", 1.0);
    diction.insert("D-", 0.7);
    diction.insert("F", 0.0);
    let mut count= 0;
    let mut result =0.0 ;                                                               0;               
    for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.split_whitespace().collect();
    let a = parts[0];
    let b: i32 = parts[1].parse().expect("Invalid input");
    let c = parts[2];
    
    result+=(b as f64) * diction[c];
    count+=b;

    
    }
    writeln!(writer,"{:.2}",((result/(count as f64)*100.0).round()/100.0)).unwrap()
}
