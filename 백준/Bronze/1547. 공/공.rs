use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};



fn main() {
    let mut v= [1,2,3];
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
     let mut writer= BufWriter::new(stdout().lock());
     reader.read_line(&mut input).unwrap();

     let n= input.trim().parse::<usize>().unwrap();

     for i in 0..n{
       input.clear();
       reader.read_line(&mut input).unwrap();
       let mut nums=input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()) ;

       let a= nums.next().unwrap();
       let b= nums.next().unwrap();
       
       v.swap(a-1, b-1)

     }
     for i in 0..v.len(){
        if 1==v[i]{
            writeln!(writer,"{}",i+1).unwrap();
        }
        
     }
   


}
