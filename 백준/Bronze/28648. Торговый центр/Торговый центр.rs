use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader =BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());

    reader.read_line(&mut input).unwrap();

    let num:i32  = input.trim().parse().unwrap();
    let mut result = 100000000;
    for i in 0..num{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let nums:Vec<i32> = input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

        let sum = nums[0]+nums[1];
        if result >sum {
            result = sum;
        }
    }
    writeln!(writer,"{}",result).unwrap();
    

}