use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();//사람 전체 수 n
    //x1>x2 ,   y>y2면 a가 덩치가 더크다
    let mut lanking:Vec<u32>= vec![0;n as usize];
    let mut people: Vec<(u32,u32)>= Vec::new();
    //배열애 삽입
    for _ in 0..n{
       input.clear();
       reader.read_line(&mut input).unwrap();
       let inputs= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<u32>>();
       let x=inputs[0];
       let y=inputs[1];
       people.push((x,y));
    }

//사람을 순회
for i in &people {
    let mut rank = 1;
    //비교해가면서 둘다 낮으면 랭크를 높이기
    for j in &people {
        if i.0 < j.0 && i.1 < j.1 {
            rank += 1;
        }
    }
    write!(writer,"{} ", rank).unwrap();
}
  writer.flush().unwrap();
}