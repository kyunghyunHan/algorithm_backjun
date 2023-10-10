use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<u32>>();
     let mut e=0;
     let mut s=0;
     let mut m=0;
     let mut count= 0;




    loop{
        e+=1;
        s+=1;
        m+=1;
        count+=1;

       
        if e==nums[0] &&  s==nums[1]&& m ==nums[2] {
            break;
        }
        
        if e==15{
            e=0;
        }
         
        if s==28{
            s=0;
        }
        if m==19{
            m=0;
        }
    }
    

    writeln!(writer,"{}",count).unwrap();

}