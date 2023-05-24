use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let mut nums = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let u = nums.next().unwrap();
    let n = nums.next().unwrap();
    
    let mut people_dict: std::collections::HashMap<usize, Vec<String>> = std::collections::HashMap::new();
    
    for _ in 0..n {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace();
        let s = nums.next().unwrap().to_string();
        let p = nums.next().unwrap().parse::<usize>().unwrap();
        if p <= u {
            people_dict.entry(p).or_insert(Vec::new()).push(s);
        }
    }
    
    let min_count = people_dict.values().map(|v| v.len()).min().unwrap();
    let min_key = people_dict
        .keys()
        .filter(|k| people_dict[k].len() == min_count)
        .min()
        .unwrap();
    
    println!("{} {}", people_dict[min_key][0], min_key);
}
