use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let num =input.trim().parse().unwrap();
    let mut result = 0;
    for i in 0..num {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut nums:Vec<i32>= input.split_whitespace().map(|x|x.parse().unwrap()).collect();
        if nums[0] * nums[1] > result {
            result =nums[0] * nums[1]
        }
        
    }
    writeln!(writer,"{}",result).unwrap();

}