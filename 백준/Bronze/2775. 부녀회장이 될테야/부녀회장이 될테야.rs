use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    let mut arr: [[i32; 15]; 15] = [[0; 15]; 15];//아파트의 층은 14층보다 작거나 같디때문에 2중배열
    
    reader.read_line(&mut input).unwrap();
    let t= input.trim().parse::<u32>().unwrap();
    for i in 0..15 {
        arr[0][i] = i as i32; //0층i호에는 i만큼 살기때문에 
    }
    for i in 1..15 {
        for j in 1..15 {
            arr[i][j] = arr[i - 1][j] + arr[i][j - 1];
        }//1호부터 해당호까지 사람들의 합을 넣어주기
    }
    for _ in 0..t{
      input.clear();
      reader.read_line(&mut input).unwrap();
     //k층에 n호에는 몇명이살고있는지 출력
     //층은 0층부터
     //층에는 1부터 i호에는 i명이산다 
     //아파트의 층과 호수는 14와 같거나 작다
      let  k: u32 = input.trim().parse::<u32>().unwrap();
     
      input.clear();
      reader.read_line(&mut input).unwrap();

      let  n: u32 = input.trim().parse::<u32>().unwrap();
      writeln!(writer,"{}", arr[k as usize][n as usize]).unwrap();

    }
    writer.flush().unwrap();
}