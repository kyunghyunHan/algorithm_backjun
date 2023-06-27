use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut writet= BufWriter::new(stdout().lock());
    let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
  /*정렬후 */
    nums.sort();

    /*현재로 귀한이 가능하다면 S 
    - 334인경우
    - 344인경우
    - 1 2 3인경우 모두가능
    
    */
    if nums[0]==nums[1]||nums[1]==nums[2]||nums[0]+nums[1]==nums[2]{
        writeln!(writet,"{}","S").unwrap();
    }else{
        writeln!(writet,"{}","N").unwrap();

    }
}
