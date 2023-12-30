 use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

 fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input= String::new();

    reader.read_line(&mut input).unwrap();
    let inputs=input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<u32>>();
    let n= inputs[0];
    let m= inputs[1];
    let mut count= 0;
    for i  in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s= input.trim();
        let mut oc= 0;
        let mut xc= 0;

        for i in s.chars(){
            if i =='O'{
                oc+=1;
            }else {
                xc+=1;
            }
        }
        if oc > xc {
           count+=1;
        }

    }

    writeln!(writer,"{}",count).unwrap();
    writer.flush().unwrap();
 }