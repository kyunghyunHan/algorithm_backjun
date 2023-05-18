use std::io::{BufReader,BufRead,BufWriter,stdin,stdout,Write};

fn main(){
let mut reader= BufReader::new(stdin().lock());

let mut writer= BufWriter::new(stdout().lock());
let mut input = String::new();
reader.read_line(&mut input).unwrap();
let  n: isize= input.trim().parse::<isize>().unwrap();

let mut result= n;
loop{
let mut input2 = String::new();

reader.read_line(&mut input2).unwrap();
let bools= input2.trim();
if bools== "="{
    writeln!(writer,"{}",result).unwrap();
    break;
}
let mut input3 = String::new();
reader.read_line(&mut input3).unwrap();
let  num= input3.trim().parse::<isize>().unwrap();



if bools=="+"{
    result+=num;
}else if bools=="-"{
    result-=num;
}else if bools=="*"{
    result*=num;
}else if bools=="/"{
    result/=num;
}

                                                                                                                                                                                                                                                                                                                          


}


}