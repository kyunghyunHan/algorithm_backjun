use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let mut n = input.trim().parse::<u64>().unwrap();
    let mut files:Vec<u64>= vec![];
    let mut result =0;
    input.clear();
    reader.read_line(&mut input).unwrap();
    
    let mut data1 = input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<u64>>();
    for i in 0..n{
        files.push(data1[i as usize])
    }
    input.clear();

        reader.read_line(&mut input).unwrap();
    let mut data2 = input.trim().parse::<u64>().unwrap();
    


   for i in 0..n{
    if   files[i as usize]>data2{
    if files[i as usize] % data2==0{
        result+=files[i as usize]/data2;
    }else{
        result+=files[i as usize]/data2 +1;
    }
    }else if files[i as usize]!=0{
        result+=1;
    }
   }
   writeln!(writer,"{}",result*data2).unwrap();
   writer.flush().unwrap();
}