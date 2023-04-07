use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
use std::collections::HashMap;

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();
    //세번째는 각  항목이 입력받은 순서
    let mut pairs:[[usize;33];33]= [[0;33];33];
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for _ in 0..n {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        let mut nums = buffer.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let n = nums.next().unwrap();
        let m = nums.next().unwrap();
        for i in 1..=m{
          pairs[1][i]=i;
        }

        for i in 2..=n{
            for j  in 2..=m{
                pairs[i as usize][j as usize]=pairs[i as usize -1  ][j as usize-1]+pairs[i as usize][j as usize-1];
            }
        }

        
       
        writeln!(writer,"{}",pairs[n as usize][m as usize]).unwrap();
        
    }




   
}