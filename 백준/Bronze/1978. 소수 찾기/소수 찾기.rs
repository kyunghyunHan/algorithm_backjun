use std::io::{BufReader,BufRead,BufWriter,stdin,Write,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n :i32= input.trim().parse().unwrap();
    input.clear();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let nums= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    let mut result= 0;
    for  i in nums {
        let mut cnt = 0;
        if i >1{
            for j in 2..i{
                if i % j==0 {
                    cnt+=1;
                }
            }
            if cnt ==0 {
                result+=1;
            }
        }
    }
     writeln!(writer,"{}",result).unwrap();
     writer.flush().unwrap();
}