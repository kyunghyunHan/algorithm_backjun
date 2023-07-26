use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main(){
    let mut reader=BufReader::new(stdin().lock());

    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let mut nums:Vec<String>= input.trim().split_whitespace().map(|x|x.to_string()).collect();
    let a:i32=nums[0].parse().unwrap();
    let b:i32=nums[2].parse().unwrap();
    let c:i32=nums[4].parse().unwrap();
    let mut result= 0;
    let mut result2= 0;

    if nums[1]=="+"{
        if nums[3]=="+"{
            result= a+b+c;
            result2= a+b+c;
        }else if nums[3]=="/"{
            result= (a+b)/c;
            result2=a+(b/c);

        }else if nums[3]=="-"{
            result= a+b-c;
            result2= a+b-c;

        }else if nums[3]=="*"{
            result= (a+b)*c;
            result2= a+(b*c);
        }
    }else if nums[1]=="/"{
        if nums[3]=="+"{
            result= (a/b)+c;
            result2= a/(b+c);

        }else if nums[3]=="/"{
            result= (a/b)/c;
            result2= a/(b/c);
        }else if nums[3]=="-"{
            result= (a/b)- c;
            result2= a/(b- c);
        }else if nums[3]=="*"{
            result= (a/b)*c;
            result2= a/(b*c);
        }
    }else if nums[1]=="-"{
        if nums[3]=="+"{
            result= (a-b)+c;
            result2= a-(b+c);
        }else if nums[3]=="/"{
            result= (a-b)/c;
            result2= a-(b/c);
        }else if nums[3]=="-"{
            result= (a-b)-c;
            result2= a-(b-c);
        }else if nums[3]=="*"{
            result= (a-b)*c;
            result2= a-(b*c);
        }
    }else if nums[1]=="*"{
        if nums[3]=="+"{
            result= (a*b)+c;
            result2= a*(b+c);
        }else if nums[3]=="/"{
            result= (a*b)/c;
            result2= a*(b/c);
        }else if nums[3]=="-"{
            result= (a*b)-c;
            result2= a*(b-c);
        }else if nums[3]=="*"{
            result= (a*b)*c;
            result2= a*(b*c);
        }
    }
    

    if result >result2 {
        writeln!(writer,"{}\n{}",result2,result).unwrap();
    }else{
        writeln!(writer,"{}\n{}",result,result2).unwrap();

    }
}