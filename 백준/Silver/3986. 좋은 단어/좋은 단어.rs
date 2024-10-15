use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write}; //원형큐
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut wirter = BufWriter::new(stdout().lock());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();
    let mut ans = 0;
    for _ in 0..n {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let s :Vec<char>= line.trim().chars().collect();
        let mut v = Vec::new();

        for i in s {
            if v.is_empty() {
                v.push(i);
            } else if *v.last().unwrap()== i {
                v.pop();
            } else {
                v.push(i);
            }
        }
        if v.is_empty() {
            ans += 1;
        }
    }
    writeln!(wirter, "{}", ans).unwrap();
    wirter.flush().unwrap();
}
