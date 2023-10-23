use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
        
    reader.read_line(&mut input).unwrap();
    let t= input.trim().parse::<u32>().unwrap();
  
    let mut arr:Vec<u32> = Vec::new();
    for _ in 0..t{
      input.clear();
      reader.read_line(&mut input).unwrap();
      let  k: u32 = input.trim().parse::<u32>().unwrap();
      arr.push(k)
    }
    let right_arr: Vec<u32> = arr.clone().into_iter().rev().collect();
    let mut right_count= 0;
    let mut left_count= 0;
    let mut right_value= 0;
    let mut left_value= 0;

    for i in 0..t{
        if arr[i as usize]>left_value{
            left_count+=1;
            left_value=arr[i as usize];

        }
       
    }
    for i in 0..t{
        if right_arr[i as usize]>right_value{
            right_count+=1;
        right_value=right_arr[i as usize];

        }
    }
    writeln!(writer,"{}",left_count).unwrap();
    writeln!(writer,"{}",right_count).unwrap();

    writer.flush().unwrap();
}