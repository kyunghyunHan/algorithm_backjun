use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let nums= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<usize>>();
    let n =nums[0];
    let m= nums[1];
    input.clear();
    reader.read_line(&mut input).unwrap();

    let a= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<usize>>();

    let mut s:Vec<usize>= vec![0;n];
    let mut c:Vec<usize>= vec![0;m];
    let mut answer=0;

    s[0] = a[0];
    for i  in 1..n{
        s[i]= s[i - 1] + a[i];
    }

    for i in 0..n{
        let remeinder= s[i]%m;

        if remeinder == 0{
            answer +=1;
        }
        c[remeinder]+=1;
    }

    for i in 0..m{
       if c[i]>1{
        answer+=(c[i]*(c[i]-1))/2
       }
    }
    writeln!(writer,"{}",answer).unwrap();
}