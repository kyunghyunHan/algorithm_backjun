use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let nums= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<isize>>();

    let mut n= nums[0] ;
    let m= nums[1];
    let M= nums[2];
    let T= nums[3];
    let R= nums[4];
   

    let mut pulse = m;
    let mut minute= 0;
    loop {
        if (M-m)<T{
            writeln!(writer,"{}","-1").unwrap();
            break;
        }
       if pulse +T<=M{
        pulse+=T;
        n-=1;
       }else {
        pulse -=R;
        if pulse <m{
            pulse=m
        }
    }
        minute+=1;

        if n ==0 {
            writeln!(writer,"{}",minute).unwrap();
            break;
        }
       }
    }

