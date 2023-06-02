use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){

    let mut reader= BufReader::new(stdin().lock());
let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let mut map= [[0;102];102];

     let n: i32= input.trim().parse().unwrap();

     for  i in 1..=n{
        input.clear();

        reader.read_line(&mut input).unwrap();
   let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
  for a in nums[0]..nums[0]+nums[2]{
    for b in nums[1]..nums[1]+nums[3]{
        map[a][b]= i;
    }
  }
     }




     for i in 1..=n{
        let mut sum= 0;
        for a in 0..101{
            for b in 0..101{
                if map[a][b]==i{
                    sum+=1;
                }
            }
        }
        writeln!(writer,"{}",sum).unwrap();
     }
}