

use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    let mut ans: i32 = 0;

    let mut k = 1;


    let mut arr: Vec<i32> = vec![0; 1001];

    for i in 1..=1000 {
        for _j in 1..=i {
            if k > 1000 {
                break;
            }
            arr[k] = i;
            k += 1;
        }
    }

    reader.read_line(&mut input).unwrap();

    //배열로 받기
    let mut nums= input.trim().split_ascii_whitespace().map(|x|x.parse::<u64>().unwrap());

    let n= nums.next().unwrap();
    let m= nums.next().unwrap();

    for i in n..=m {
        ans += arr[i as usize];
    }
    writeln!(writer,"{}",ans).unwrap();
    writer.flush().unwrap();

}