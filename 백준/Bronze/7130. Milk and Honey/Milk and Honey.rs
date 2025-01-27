use std::{
    cmp::max,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let mh = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let m = mh[0]; //우유 단위당 행복도의양
        let h = mh[1]; //꿀 1 단위당 행복의 양
        if let Some(Ok(line)) = input.next() {
            let n: usize = line.parse::<usize>().unwrap();
            let mut result = 0;
            for _ in 0..n {
                if let Some(Ok(line)) = input.next() {
                    let cb = line
                        .trim()
                        .split_whitespace()
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    let c = cb[0]; //밭에서 키울수 있는 소의 수
                    let b = cb[1]; //밭에서 키울수 있는 꿀벌의 수

                    result += max(c * m, b * h)
                }
            }
            writeln!(writer, "{}", result).unwrap();
        }
    }
    writer.flush().unwrap();
}
