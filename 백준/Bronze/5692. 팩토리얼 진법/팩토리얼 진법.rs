use std::io::{BufRead,BufReader,Write,stdin,stdout,BufWriter};


fn factorial(num:usize)->usize{
    if num ==1{
        1
    }else{
        num*factorial(num-1)
    }

}
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());


    loop{
        let mut input= String::new();
        reader.read_line(&mut input).unwrap();
    
        let n: &str = input.trim();
        let mut length = n.len(); 
        let mut result= 0;
        if n =="0"{
          break;
        }
        for i in n.chars(){
            result+= ( i.to_string().parse::<usize>().unwrap() * factorial(length)); 
            length-= 1;
         }
         writeln!(writer,"{}",result).unwrap();
    }
}   