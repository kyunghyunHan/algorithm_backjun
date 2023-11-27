use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let n:u64= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();

    let k:u64= input.trim().parse().unwrap();


    let mut s = vec![0; n as usize + 1];

    for i in 2..=n {
        if s[i as usize] == 0 {
            let mut t = i;
            while t <= n {
                if t % i == 0 {
                    s[t as usize] = i;
                }
                t += i;
            }
        }
    }
    let mut ans= 0;
    for i in s{
        if i <=k{
            ans+=1;
        }
    }
writeln!(writer,"{}",ans-1).unwrap();
writer.flush().unwrap();
}