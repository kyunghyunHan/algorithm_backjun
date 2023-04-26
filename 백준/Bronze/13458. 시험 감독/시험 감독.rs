use std::io::{stdin,stdout, BufRead, BufReader, BufWriter, Write};


fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();
    
    let mut buffer2= String::new();
    reader.read_line(&mut buffer2).unwrap();
    let n2: Vec<i32> = buffer2
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect();

    let mut buffer3= String::new();
    reader.read_line(&mut buffer3).unwrap();
    let mut n3_v= buffer3.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());

    let mut n3_a= n3_v.next().unwrap();
    let mut n3_b= n3_v.next().unwrap();
    let mut ans: usize = 0;
    for i in 0..n {
        let mut k = 1;
        if n2[i ] > n3_a as i32 {
            k += (n2[i] - n3_a as i32) / n3_b as i32;
            if (n2[i] - n3_a as i32) % n3_b as i32 > 0 {
                k += 1;
            }
        }
        ans += k as usize;
    }

   writeln!(writer,"{}",ans).unwrap();
}
