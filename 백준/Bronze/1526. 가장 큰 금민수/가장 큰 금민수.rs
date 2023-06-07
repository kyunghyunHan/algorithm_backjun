use std::io::{BufWriter,BufRead,BufReader,Write,stdin,stdout};

fn main() {
 let mut reader= BufReader::new(stdin().lock());
 let mut input = String::new();
let mut writer= BufWriter::new(stdout().lock());
  reader.read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let mut temp: u32;
    let mut flag: bool = false;
  //입력받은수부터 1씩 감소 하면서 탐색
    for i in (0..=n).rev() {
        temp = i;
        flag = true;
    
        while temp != 0 {
            if temp % 10 != 4 && temp % 10 != 7 {
                flag = false;
                break;
            } else {
                temp /= 10;
            }
        }
//flag
        if flag {
            println!("{}", i);
            break;
        }
    }
}
