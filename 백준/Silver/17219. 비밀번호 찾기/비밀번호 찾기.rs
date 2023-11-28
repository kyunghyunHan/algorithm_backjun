use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
use std::collections::HashMap;
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    
    let mut nm = input.trim().split_whitespace();
    let n:i32= nm.next().unwrap().parse().unwrap();
    let m :i32=nm.next().unwrap().parse().unwrap();
    let mut database= HashMap::new();
    for _ in 0..n{
        let mut sitemap= String::new();
        reader.read_line(&mut sitemap).unwrap();
         let mut s = sitemap.trim().split_whitespace();
         let site= s.next().unwrap().to_string();
         let  password= s.next().unwrap().to_string();
         database.insert(site, password);
    }
    for _ in 0..m{
        let mut sitemap = String::new();
        reader.read_line(&mut sitemap).unwrap();
        let site= sitemap.trim().to_string();
        writeln!(writer,"{}",database.get(&site).unwrap()).unwrap();
    }
writer.flush().unwrap();
}