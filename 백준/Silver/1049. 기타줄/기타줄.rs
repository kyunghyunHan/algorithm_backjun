use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();

    let  inputs= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    let n= inputs[0];
    let m= inputs[1];
    let mut min_pack = 1001; 
    let mut min_single = 1001;
    for _ in 0..m{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let  inputs= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
        let p= inputs[0];
     
        let s= inputs[1];
        min_pack = min_pack.min(p);
        min_single = min_single.min(s);
        
    }
    let mut result = 0;

    if min_pack > min_single * 6 {
        result += n * min_single;
    } else {
        result += (n / 6) * min_pack;

        if (n % 6) * min_single > min_pack {
            result += min_pack;
        } else {
            result += (n % 6) * min_single;
        }
    }
    writeln!(writer,"{}", result).unwrap();
    writer.flush().unwrap();

}