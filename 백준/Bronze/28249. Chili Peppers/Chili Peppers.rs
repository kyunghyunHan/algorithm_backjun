use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    const Pobolano:i32 = 1500;
    const Mirasol:i32 = 6000;
    const Serrano:i32 = 15500;
    const Cayenne:i32 = 40000;
    const Thai:i32 = 75000;
    const Habanero:i32 = 125000;

    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());

    reader.read_line(&mut input).unwrap();

    let num :i32= input.trim().parse().unwrap();
    let mut result  = 0;
    for i in 0..num{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let text = input.trim();

        if text =="Poblano" {
            result +=Pobolano;
        }else if text =="Mirasol" {
            result +=Mirasol;

        }else if text =="Serrano" {
            result +=Serrano;

        }else if text =="Cayenne" {
            result +=Cayenne;

        }else if text =="Thai" {
            result +=Thai;

        }else if text =="Habanero" {
            result +=Habanero;

        }

    }
    writeln!(writer,"{}",result).unwrap();

}