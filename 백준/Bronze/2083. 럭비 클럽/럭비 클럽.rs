use std::io::{BufWriter,BufReader,BufRead,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());



let mut writer= BufWriter::new(stdout().lock());

loop{
    let mut input = String::new();
reader.read_line(&mut input).unwrap();
let mut user= input.trim().split_whitespace().map(|x|x);
let  user_name= user.next().unwrap();
let  num= user.next().unwrap().parse::<usize>().unwrap();
let user_kg= user.next().unwrap().parse::<usize>().unwrap();

if user_name=="#" && num==0 {
    break;
}

if num >17 ||user_kg>=80{
 writeln!(writer,"{} {}",user_name,"Senior").unwrap();
}else{
writeln!(writer,"{} {}",user_name,"Junior").unwrap();
} 

}
}