use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let mut nums:Vec<&str>= input.trim().split_whitespace().map(|x|x).collect();
    let min_sum: i32 = nums[0].trim().replace('6', "5").parse::<i32>().unwrap()
    + nums[1].trim().replace('6', "5").parse::<i32>().unwrap();

// Replacing '5' with '6' and calculating max
    let max_sum: i32 = nums[0].trim().replace('5', "6").parse::<i32>().unwrap()
    + nums[1].trim().replace('5', "6").parse::<i32>().unwrap();

    writeln!(writer,"{} {}", min_sum, max_sum).unwrap()

}