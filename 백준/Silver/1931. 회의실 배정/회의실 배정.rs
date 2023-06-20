use std::io::{BufReader,BufRead,BufWriter,stdin,stdout,Write};
/*
각 회의가 겹쳐지지 않게 하기
회의는 한번 시작하면 중간에 중단될수 없다
작업스케줄

 */
fn main(){
    let mut reader=BufReader::new(stdin().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let mut writer= BufWriter::new(stdout().lock());


    let n= input.trim().parse().unwrap();
    let mut v:Vec<(i32,i32)>= vec![];
    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let nums: Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
        

        v.push((nums[1],nums[0]));
    }
    v.sort();
    let mut time = v[0].0;
    let mut count = 1;
    for i in 1..n as usize {
        if time <= v[i].1 {
            count += 1;
            time = v[i].0;
        }
    }
writeln!(writer,"{}",count).unwrap();
}

