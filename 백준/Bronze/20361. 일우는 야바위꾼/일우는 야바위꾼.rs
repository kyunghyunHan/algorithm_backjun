use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};

fn main(){

   let mut reader= BufReader::new(stdin().lock());
   let mut buffer= String::new();
   reader.read_line(&mut buffer).unwrap();
   let mut nums=buffer.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()) ;
   let mut writer= BufWriter::new(stdout().lock());
    //n개의 숫자를
   let n=nums.next().unwrap();
   //x번쨰가 공
   let x: usize=nums.next().unwrap();
   //k번실행
   let k=nums.next().unwrap();
let mut v: Vec<usize>= vec![  ];

for i in 1..=n{
   v.push(i)
}
for i  in 0..k{
   buffer.clear();
   reader.read_line(&mut buffer).unwrap();
   let mut nums=buffer.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()) ;
   let a=nums.next().unwrap();
   let b=nums.next().unwrap();
   v.swap(a-1, b-1)
}

for i in 0..v.len(){
   if v[i]==x{
      writeln!(writer,"{:?}",i+1).unwrap();
   }
}

}