use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();
reader.read_line(&mut input).unwrap();

let mut nums:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
input.clear();
reader.read_line(&mut input).unwrap();
let mut nums2:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

let mut sun_year= nums[1]-nums[0];
let mut moon_year= nums2[1]-nums2[0];

while sun_year !=moon_year {
    if sun_year<moon_year {
        sun_year+=nums[1]
    }else {
        moon_year+=nums2[1]
    }
}
writeln!(writer,"{}",sun_year).unwrap();
}
