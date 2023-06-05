use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){

    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
   let n :u128= input.trim().parse().unwrap();
   let mut result =1;
   for i in 2..=n {
    result *= i;
    //0인지 확인하기위해
    //0이라면 0이 아닐까지 반복
    while result % 10 == 0 {
        result /= 10;
    }
    //오버플로우 방지
    result %= 100000000000000000;
}
writeln!(writer,"{}", result.to_string().chars().rev().take(5).collect::<String>().chars().rev().collect::<String>()).unwrap();

}
