use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().expect("Please enter a valid integer");

    for case in 0..n {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let mut nums: Vec<i32> = input.split_whitespace()
                                       .map(|x| x.trim().parse().expect("Invalid input"))
                                       .collect();
        nums.sort();

        print!("Case #{}: ", case + 1);
        if nums[0] + nums[1] <= nums[2] {
            println!("invalid!");
        } else if nums[0] == nums[1] && nums[1] == nums[2] {
            println!("equilateral");
        } else if nums[0] == nums[1] || nums[1] == nums[2] || nums[2] == nums[0] {
            println!("isosceles");
        } else {
            println!("scalene");
        }
    }
    writer.flush().unwrap();
}