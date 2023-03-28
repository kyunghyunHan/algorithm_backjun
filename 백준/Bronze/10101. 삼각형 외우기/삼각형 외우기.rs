use std::io::{stdin, BufRead, BufReader, BufWriter, Write};


fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    
    let mut arr :Vec<usize>= vec![];
    for _ in 0..3{
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        let n: usize = buffer.trim().parse().unwrap();

        arr.push(n);
    }
    
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    if arr[0]+arr[1]+arr[2]==180{
        if arr[0]==60 && arr[1]==60&&arr[2]==60{
            writeln!(writer,"{}","Equilateral").unwrap();
        }else if arr[0]==arr[1] || arr[1]==arr[2]|| arr[2]==arr[0]{
            writeln!(writer,"{}","Isosceles").unwrap();
        }else{
            writeln!(writer,"{}","Scalene").unwrap();
        }
    }else{
        writeln!(writer,"{}","Error").unwrap();
    }
  
}
