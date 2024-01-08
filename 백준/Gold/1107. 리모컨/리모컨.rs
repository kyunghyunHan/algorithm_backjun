use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let target:i32= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let n:usize= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();

    let broken: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut min_count = (100 - target).abs();

    for nums in 0..=1000000 {
        let nums_str = nums.to_string();

        for (j, num_char) in nums_str.chars().enumerate() {
            let num = num_char.to_digit(10).unwrap() as i32;

            if broken.contains(&num) {
                break;
            } else if j == nums_str.len() - 1 {
                let abs_diff = (nums - target).abs() + nums_str.len() as i32;
                min_count = min_count.min(abs_diff);
            }
        }
    }

    writeln!(writer,"{}", min_count).unwrap();
    writer.flush().unwrap();
}
