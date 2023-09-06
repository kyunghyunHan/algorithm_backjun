use std::io::{BufRead,BufReader,Write,stdin,stdout,BufWriter};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
   

    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let  weather:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();



    input.clear();
    reader.read_line(&mut  input).unwrap();
    let  tomorrow_weather:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    
    if tomorrow_weather[0] <0 && tomorrow_weather[1]>=10{
        writeln!(writer,"{}","A storm warning for tomorrow! Be careful and stay home if possible!").unwrap();
    }else if tomorrow_weather[0]<weather[0]{
        writeln!(writer,"{}","MCHS warns! Low temperature is expected tomorrow.").unwrap();

    }else if tomorrow_weather[1]>weather[1]{
        writeln!(writer,"{}","MCHS warns! Strong wind is expected tomorrow.").unwrap();

    }else {
        writeln!(writer,"{}","No message").unwrap();

    }

    
}