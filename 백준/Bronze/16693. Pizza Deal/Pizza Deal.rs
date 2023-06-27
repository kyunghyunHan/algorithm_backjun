use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut writet= BufWriter::new(stdout().lock());
    let mut a_pizza:Vec<f64>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
     

     input.clear();
     reader.read_line(&mut input).unwrap();
     let mut b_pizza:Vec<f64>= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();

      
    

    let a_pizze_result =a_pizza[0]/a_pizza[1];
    let b_pizze_result=  b_pizza[0] * b_pizza[0]* 3.14159265359 / b_pizza[1];

    if a_pizze_result<b_pizze_result{
   writeln!(writet,"{}","Whole pizza").unwrap();
    }else{
        writeln!(writet,"{}","Slice of pizza").unwrap();

    }
}
