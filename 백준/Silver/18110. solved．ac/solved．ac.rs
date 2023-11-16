use std::{io::{BufReader,BufRead,BufWriter,Write,stdin,stdout}, iter};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    //난이도 의견의 개수 n
    let n:i32= input.trim().parse().unwrap();
    //15%
    if n == 0
	{
    writeln!(writer,"{}",0).unwrap();
    writer.flush().unwrap();

    std::process::exit(0);
	}

    let cut_num=   (n as f32 *15.0/100.0).round() as usize;
    let mut v= Vec::new();
    for _ in 0..n{
       input.clear();
       reader.read_line(&mut input).unwrap();
       let m:f32= input.trim().parse().unwrap();
       v.push(m)
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());    let  result_sum:&f32= &v[cut_num as usize..=((v.len()-1)-(cut_num as usize))].iter().sum();
    writeln!(writer,"{}",(result_sum/v[cut_num as usize..=((v.len()-1)-(cut_num as usize))].len() as f32).round()as i32).unwrap();
    writer.flush().unwrap();
}