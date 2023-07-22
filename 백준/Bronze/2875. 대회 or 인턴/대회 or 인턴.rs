use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();

    reader.read_line(&mut input).unwrap();
    
    let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

    for i in 0..nums[2]{
        if nums[0]/2 >=nums[1]{
            nums[0]-=1;
        }else {
            nums[1]-=1;
        }
    }
    writeln!(writer,"{}",usize::min(nums[0]/2, nums[1])).unwrap();
}