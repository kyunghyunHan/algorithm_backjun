use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){

    

let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
let mut input= String::new();
reader.read_line(&mut input).unwrap();


let nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

if (nums[1]-nums[0])%nums[2]!=0{
    writeln!(writer,"{}",((nums[1]-nums[0])/nums[2])+1).unwrap();

}else{
    writeln!(writer,"{}",(nums[1]-nums[0])/nums[2]).unwrap();

}
}
