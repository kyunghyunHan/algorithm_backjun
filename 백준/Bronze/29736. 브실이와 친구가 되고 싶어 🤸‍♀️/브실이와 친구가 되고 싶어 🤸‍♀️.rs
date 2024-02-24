use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let  ab: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let  kx: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut cnt = 0;
    for _i in kx[0] - kx[1]..kx[0]+kx[1]+1 {
        if ab[0] <= _i && ab[1] >= _i {
            cnt += 1;
        }
    }
    if cnt == 0 {
        writeln!(writer, "{}", "IMPOSSIBLE").unwrap();
    } else {
        writeln!(writer, "{}", cnt).unwrap();
    }
    writer.flush().unwrap();
}
