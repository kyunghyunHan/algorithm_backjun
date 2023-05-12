use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){

    let mut reader= BufReader::new(stdin().lock());

    let mut writer= BufWriter::new(stdout().lock());
    loop{ 
       
        let mut input= String::new();
        reader.read_line(&mut input).unwrap();
        let mut nums= input.trim().split_whitespace().map(|x|x.parse::<isize>().unwrap());

        let a1= nums.next().unwrap();
        let a2= nums.next().unwrap();
        let a3= nums.next().unwrap();
        if a1==0 &&a2==0&&a3==0{
            break;
        }
         if a3 - a2 == a2 - a1{
            writeln!(writer,"AP {}",a3+(a3-a2)).unwrap();
         }else{
            writeln!(writer,"GP {}",a3*(a3/a2)).unwrap();
         }
    }

}