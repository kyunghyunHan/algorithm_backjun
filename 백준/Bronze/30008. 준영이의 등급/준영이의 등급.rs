use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let nk = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let gv = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    //n:학생의 수
    //과목의수 :k
    let mut jv = gv.iter().map(|x| x * 100 / nk[0]).collect::<Vec<i32>>();

    let mut rv = Vec::new();
    for i in jv {
        if 0 <= i && i <= 4 {
            rv.push(1);
        } else if 4 < i && i <= 11 {
            rv.push(2);
        } else if 11 < i && i <= 23 {
            rv.push(3);
        } else if 23 < i && i <= 40 {
            rv.push(4);
        } else if 40 < i && i <= 60 {
            rv.push(5);
        } else if 60 < i && i <= 77 {
            rv.push(6);
        } else if 77 < i && i <= 89 {
            rv.push(7);
        } else if 89 < i && i <= 96 {
            rv.push(8);
        } else if 96 < i && i <= 100 {
            rv.push(9);
        }
    }
    for i in rv{
      print!("{} ",i);
    }
}
