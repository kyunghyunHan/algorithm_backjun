use std::io::{stdin, BufRead, BufReader, BufWriter, Write};

fn fib(n:usize,cnt:&mut usize)->usize{
    if n==1||n==2{
        *cnt+=1;
        return 1;
    }else{
        return fib(n-1,cnt)+fib(n-2,cnt);
    }
}
fn main() {
     let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut arr: Vec<usize>=vec! [0;41];
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap(); // 수정된 부분
    let mut nums = buffer.trim().split_whitespace();
    let n = nums.next().unwrap().parse::<usize>().unwrap();
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let mut cnt:usize=0;
    let mut cnt2= 0;
    fib(n, &mut cnt);


    arr[1]=1;
    arr[2]=1;

    for i in 3..=n{
        arr[i]= arr[i-1]+arr[i-2];
        cnt2+=1;
    }
   
   writeln!(writer,"{} {}",cnt,cnt2).unwrap();
}
