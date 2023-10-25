use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let s: &str= input.trim();

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: i32= input.trim().parse().unwrap();
    let mut cnt = 0;

    for _ in 0..n {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
       //to_string은 포맷팅함수를 사용하기때문에 buffer을 할당하고 리터럴을 버퍼에 복사해 넣는 to_owned사용하는게 좋ㅎ음
        let ring_string = input.trim().to_owned();
        let doubled_ring = ring_string.clone() + &ring_string;
         //물자열찾는
        if doubled_ring.contains(s) {
            cnt += 1;
        }
    }
    writeln!(writer,"{}",cnt).unwrap();
writer.flush().unwrap();
    
}
