use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();

    let n:i32 = input.trim().parse().unwrap();
    
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut nums:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let mut count= 0;
    let mut result = 0;
    for i in 0..n{
     

        if nums[i as usize] ==1 {
           count +=1;
           result+=count;
        }else{
            count=0;
        }
    }
    writeln!(writer,"{}",result).unwrap();
}