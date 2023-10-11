use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input =  String::new();
    let mut writer= BufWriter::new(stdout().lock());

    reader.read_line(&mut input).unwrap();
    let mut nums:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    

    let n= nums[0];
    let m =nums[1];
    let mut s= Vec::new();
    fn dfs(n: i32, m: i32, mut s: &mut Vec<i32>, start: i32) {
        if s.len() == m as usize {
            println!("{}", s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        }

        for i in start..=n {
            if !s.contains(&i) {
                s.push(i);
                dfs(n, m, &mut s, i + 1);
                s.pop();
            }
        }
    }

    dfs(n, m, &mut s, 1);
    writer.flush().unwrap();}