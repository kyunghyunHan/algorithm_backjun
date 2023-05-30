use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
   let mut map:[[bool;101];101]= [[false;101];101];
   let mut writer= BufWriter::new(stdout().lock());
     let mut reader= BufReader::new(stdin().lock());
for i in 0..4{

    let mut input = String::new();

    reader.read_line(&mut input ).unwrap();
    

    let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    for i in nums[0]..nums[2]{
     for j in nums[1]..nums[3]{
        map[i][j]= true
     }
    }
}
    

    let mut result =0;
    for i in 0..101{
        for j in 0..101{
            if map[i][j]==true{
                result+=1;
            }
        }
    }
   writeln!(writer,"{}",result).unwrap();

}