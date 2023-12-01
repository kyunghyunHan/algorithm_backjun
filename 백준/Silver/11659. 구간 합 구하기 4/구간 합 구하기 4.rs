use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};

fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());

let mut input = String::new();
reader.read_line(&mut input).unwrap();

let  nums:Vec<usize>= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect();

let n = nums[0];
let m = nums[1];

input.clear();
reader.read_line(&mut input).unwrap();
//구간합 저장
let   list:Vec<i32>= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect();
let mut result_sum = vec![0; n];
let mut sum = 0;
for (i, &num) in list.iter().enumerate() {
    sum += num;
    result_sum[i] = sum;
}
//m개의 줄
for _ in 0..m{
    input.clear();
    reader.read_line(&mut input).unwrap();
    //ij
    let ij:Vec<usize>= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect();
    
    let i = ij[0];
    let j = ij[1];

    let result = if i == 1 {
        result_sum[j - 1]
    } else {
        result_sum[j - 1] - result_sum[i- 2]
    };
    writeln!(writer, "{}", result).unwrap();

}
writer.flush().unwrap();
}

