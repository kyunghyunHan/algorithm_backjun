use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    // 카운팅 배열 초기화
    let mut counting: [usize; 10001] = [0; 10001];
    // 카운팅 정렬 (입력)
    for _ in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let n:usize= input.trim().parse().unwrap();
        counting[n]+=1;
    }
    // 카운팅 정렬 (출력)
    for i in 0..10001 {
        for _ in 0..counting[i] {
            writeln!(writer, "{}", i).unwrap();
        }
    }
writer.flush().unwrap();
    
}
