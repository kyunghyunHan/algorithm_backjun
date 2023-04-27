use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let a = nums.next().unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let mut nums2 = input2.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let b = nums2.next().unwrap();
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).unwrap();
    let mut nums3 = input3.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let c = nums3.next().unwrap();
    println!("The 1-3-sum is {}", 91 + a * 1 + b * 3 + c * 1);
}