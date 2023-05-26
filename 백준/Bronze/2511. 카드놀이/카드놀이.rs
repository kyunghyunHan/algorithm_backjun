use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){

    let mut reader= BufReader::new(stdin().lock());
    let mut input = String ::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(& mut input).unwrap();
    let mut  nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
     let mut last= 0;
    let mut a= 0;
    let mut b= 0;
    input.clear();
    reader.read_line(& mut input).unwrap();

    let mut  nums2= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    for i in 0..10{
       if nums[i]>nums2[i]{
        a+=3;
        last=1
       }else if nums[i]<nums2[i]{
        b+=3;
        last=2
       }else if nums[i]==nums2[i]{
        a+=1;
        b+=1;
       }

    }
   
    
    writeln!(writer,"{} {}",a,b).unwrap();
   if a>b{
    writeln!(writer,"{}","A").unwrap();
   }else if b>a{
    writeln!(writer,"{}","B").unwrap();
   }else if a==b{
     if last==1{
        writeln!(writer,"{}","A").unwrap();

     }else if last==2{
        writeln!(writer,"{}","B").unwrap();

     }else if last==0{
        writeln!(writer,"{}","D").unwrap();

    }
   }
   
}

