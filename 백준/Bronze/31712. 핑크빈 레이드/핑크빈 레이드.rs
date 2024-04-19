use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let y= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let d= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let p= input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut  v= Vec::new();
    v.push(y);
    v.push(d);
    v.push(p);
    let mut h:i32= input.trim().parse().unwrap();
    let mut second= 0;
    loop{
        for i in &v{
            let c= i[0];
            let d= i[1];
            if second % c==0{
                h-=d;
            }
    
        }
        if h<=0{
            writeln!(writer,"{}",second).unwrap();
            break;
        }else{
            second += 1
        }
    }
    writer.flush().unwrap();
}
