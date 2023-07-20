use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer: BufWriter<std::io::StdoutLock<'_>>= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();

    let n = input.trim().parse().unwrap();

    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).filter(|x| x % 2 == 0).collect();
         nums.sort();
         let min_num= nums[0];
         let total_sum: usize = nums.iter().sum();

        writeln!(writer,"{} {}",total_sum,min_num).unwrap();
        
    }
}   