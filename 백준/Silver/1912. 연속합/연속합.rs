use std::cmp::{max, min};
use std::io::{stdin, Read, Write, stdout, BufWriter};

const N_MAX: usize = 100_000;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut nums = [0; N_MAX+1];
    let mut buffer: Vec<i32> = Vec::new();
    let mut maximum = i32::MIN;
    let mut has_positive = false;
    for i in 0..n {
        nums[i] = input.next().unwrap();
        if nums[i] > maximum {
            maximum = nums[i];
            if !has_positive && nums[i] > 0 {
                has_positive = true;
            }
        }
    }

    if !has_positive {
        println!("{maximum}");
        return;
    }

    let mut taking_positive = nums[0] > 0;
    let mut partial_sum = 0;
    for num in nums.iter().take(n) {
        if taking_positive == (*num > 0) {
            partial_sum += num;
        } else {
            taking_positive = *num > 0;
            buffer.push(partial_sum);
            partial_sum = *num;
        }
    }
    if partial_sum > 0 {
        buffer.push(partial_sum);
    }

    let mut prev_positive = *buffer.last().unwrap();
    maximum = max(prev_positive, maximum);
    let mut i = (buffer.len() - 3) as i32;
    while i >= 0 {
        if buffer[(i+1) as usize] + prev_positive > 0 {
            buffer[i as usize] += buffer[(i+1) as usize] + prev_positive;
        }
        maximum = max(maximum, buffer[i as usize]);
        prev_positive = buffer[i as usize];
        i -= 2;

    }

    println!("{maximum}");
}