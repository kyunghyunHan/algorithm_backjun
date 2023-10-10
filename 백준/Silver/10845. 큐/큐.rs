use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let mut n:u64= input.trim().parse().unwrap();
    let mut v:Vec<i32>= vec![];
   for i in 0..n {
      input.clear();
      reader.read_line(&mut input).unwrap();
      let mut nums= input.trim().split_whitespace().map(|x|x.to_string()).collect::<Vec<String>>();
      if nums[0]=="push"{
           v.push(nums[1].parse::<i32>().unwrap());
      }else if nums[0]=="front"{
        if v.len()==0 {
            writeln!(writer,"{}","-1").unwrap();

        }else {
            writeln!(writer,"{}",v[0]).unwrap();

        }
      }else if nums[0]=="back"{

        if v.len()==0 {
            writeln!(writer,"{}","-1").unwrap();

        }else {
            writeln!(writer,"{}",v[v.len()-1 as usize]).unwrap();

        }
    }else if nums[0]=="size"{
        writeln!(writer,"{}",v.len()).unwrap();
    }else if nums[0]=="empty"{
        if v.len()==0 {
            writeln!(writer,"{}","1").unwrap();

        }else {
            writeln!(writer,"{}","0").unwrap();

        }
    }else if nums[0]=="pop"{
        if v.len()==0 {
            writeln!(writer,"{}","-1").unwrap();

        }else {
            writeln!(writer,"{}",v[0]).unwrap();

            v.remove(0);
            
        }

    }
   }
    
}