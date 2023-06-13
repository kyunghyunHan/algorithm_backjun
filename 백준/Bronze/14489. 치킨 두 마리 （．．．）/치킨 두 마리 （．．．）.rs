use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){

    let mut reader= BufReader::new(stdin().lock());

    let mut input =String::new();
    reader.read_line(&mut input).unwrap();
    
    let mut writer= BufWriter::new(stdout().lock());

    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    
    input.clear();

    reader.read_line(&mut input).unwrap();
  

   let n:usize= input.trim().parse().unwrap();
 //치킨값이 잔고보다 많으면
   if n*2 >nums[0]+nums[1]{
writeln!(writer,"{}",nums[0]+nums[1]).unwrap();
   }else  if n*2 <=nums[0]+nums[1]{
    writeln!(writer,"{}",(nums[0]+nums[1])-n*2).unwrap();

}
}

