use std::io::{BufReader,BufRead,BufWriter,stdin,Write,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input =String::new();
    reader.read_line(&mut input).unwrap();
    let  n:i32= input.trim().parse().unwrap();
    let mut stack:Vec<i32>=vec![];
    for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let  n= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
        if n[0]==1{
           stack.push(n[1])
        }else if n[0]==2{
            if stack.len()!=0{
                    writeln!(writer,"{}",stack[stack.len()-1]).unwrap();
                    stack.remove(stack.len()-1 );
            }else {
                writeln!(writer,"{}",-1).unwrap();

            }
        }else if n[0]==3{
            writeln!(writer,"{}",stack.len()).unwrap();

        }else if n[0]==4{
            if stack.len()!=0{
                writeln!(writer,"{}",0).unwrap();
        }else {
            writeln!(writer,"{}",1).unwrap();

        }
        }else if n[0]==5{
            if stack.len()!=0{
                writeln!(writer,"{}",stack[stack.len()-1]).unwrap();
        }else {
            writeln!(writer,"{}",-1).unwrap();

        }
        }
    
    }
    writer.flush().unwrap();

}
