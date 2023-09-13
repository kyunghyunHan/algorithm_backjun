use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut  writer= BufWriter::new(stdout().lock());
    

    loop{
        let mut input= String::new();

        reader.read_line(&mut input).unwrap();
        let  num = input.trim();
        if num =="0"{
            break;
        }
        let mut result = 0;
        for i in num.chars() {
             if  i =='1'{
                result+=2;
             }else if i == '0' {
                result +=4
             }else{
                result +=3
             }
        }
        result+=  num.len()+1;
        writeln!(writer,"{}",result).unwrap();
    }
}