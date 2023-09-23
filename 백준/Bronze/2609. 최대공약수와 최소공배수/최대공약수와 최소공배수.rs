use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();

    let mut nums= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<i32>>();

    let mut n1= nums[0];
    let mut n2= nums[1];

    writeln!(writer,"{}",gcd( n1,  n2)).unwrap();
    writeln!(writer,"{}",lcm( n1,  n2)).unwrap();

    writer.flush().unwrap();

    
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}
