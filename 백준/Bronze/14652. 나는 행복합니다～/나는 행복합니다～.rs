use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};


fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut input =String::new();
    reader.read_line(&mut input).unwrap();
   let mut writer= BufWriter::new(stdout().lock());

   let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

   writeln!(writer,"{} {}",nums[2]/nums[1],nums[2]%nums[1]).unwrap();
    

}

