use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();
reader.read_line(&mut input).unwrap();
let mut nums :Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

input.clear();
reader.read_line(&mut input).unwrap();
let mut nums2 :Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();


let h1: i32 = nums[0];
let b1: i32 = nums[1];
let h2: i32 = nums2[0];
let b2: i32 = nums2[1];

let player1_score = 3 * h1 + b1;
    let player2_score = 3 * h2 + b2;
    let mut ans = String::from("NO SCORE");
    if player1_score > player2_score {
        ans = format!("1 {}", player1_score - player2_score);
    } else if player2_score > player1_score {
        ans = format!("2 {}", player2_score - player1_score);
    }
writeln!(writer,"{}",ans).unwrap();
}
