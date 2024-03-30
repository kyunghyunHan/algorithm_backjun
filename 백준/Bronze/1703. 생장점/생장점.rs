use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    loop{
        let mut input= String::new();
        reader.read_line(&mut input).unwrap();
        let li = input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
        if li[0]==0{
            break;
        }else{
            let mut n = 1;
        for i in 0..li[0] {
            let sf = li[2*i as usize + 1];
            let p = li[2*i as usize + 2];
            n = n * sf - p;
        }
        writeln!(writer,"{}", n).unwrap();
        }
    }
}
