use std::io::{BufWriter,BufReader,BufRead,Write,stdin,stdout};
fn main(){
let mut reader= BufReader::new(stdin().lock());


let mut input = String::new();
reader.read_line(&mut input).unwrap();

let n= input.trim().parse::<usize>().unwrap();
let mut writer= BufWriter::new(stdout().lock());
for i in 0..n{
    input.clear();
    reader.read_line(&mut input).unwrap();
    let n2= input.trim().parse::<usize>().unwrap();
    let mut p1= 0;
    let mut p2=0;
    for j in 0..n2{
       input.clear();
       reader.read_line(&mut input).unwrap();
       let mut prs= input.trim().split_whitespace().map(|x|x);
       let ps1= prs.next().unwrap();
       let ps2= prs.next().unwrap();
   //바위
       if ps1 =="R" {
        //보
          if ps2=="P"{
           
           p2+=1;
           //가위
            }else if ps2=="S"{
                p1+=1;
            }
 //보
       }else if ps1 =="P"{
        //바위
      if ps2=="R"{
  p1+=1;
            }     else if ps2=="S"{
                p2+=1;
            }
             //가위
       }else if ps1 =="S"{

        //바위
        if ps2=="R"{
            p2+=1;
//보
        }else if ps2=="P"{
       p1+=1;

        }    
       }



       

    }

    if p1>p2{
 writeln!(writer,"{}","Player 1").unwrap();
    }else if p1<p2{
        writeln!(writer,"{}","Player 2").unwrap();
    }else if p1==p2{
        writeln!(writer,"{}","TIE").unwrap();
    }
}
}