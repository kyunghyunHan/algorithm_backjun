use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){



let mut reader= BufReader::new(stdin().lock());

let mut input = String::new();

reader.read_line(&mut input).unwrap();
let alpabat= input.trim();
let mut count= 0;
for  i in alpabat.chars(){
    if i =='a' || i=='e' ||i=='i'||i=='o'||i=='u'{
count+=1;
    }
}
println!("{}",count);

}

