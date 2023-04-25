use std::io::{stdin,stdout,BufRead,BufReader,BufWriter,Write};


fn main(){
let stdin= stdin();
let stdout= stdout();
//도전의 가치 저장
let currency= [500,100,50,10,5,1];
//count
let mut count = 0;
let mut index= 0;
let mut reader= BufReader::new(stdin.lock());
let mut buffer= String::new();
reader.read_line(&mut buffer).unwrap();

let mut nums= buffer.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());
let  input= nums.next().unwrap();
let mut n =1000-input;
let mut writer= BufWriter::new(stdout.lock());
while n!=0{
count += n / currency[index as usize];
n %=currency[index as usize];
index+=1;
}
writeln!(writer,"{}",count).unwrap();


}