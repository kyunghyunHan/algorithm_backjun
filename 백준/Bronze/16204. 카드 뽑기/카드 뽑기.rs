use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());
let mut input= String::new();
let mut writer= BufWriter::new(stdout().lock());
reader.read_line(&mut input).unwrap();
let mut nums:Vec<usize>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();


/*
ox카드가 적혀있는 카드 n개
n개의 카드중 m개의 카드 앞면에는 O가 한개
나머지 카드의 앞면에는 x

4개 중  3개 
2 2



 */

let n= nums[0];
let m= nums[1];
let k= nums[2];

let ans = if m > k {
    k
} else {
    m
} + if n - m > n - k {
    n - k
} else {
    n - m
};
writeln!(writer,"{}",ans).unwrap();
}
