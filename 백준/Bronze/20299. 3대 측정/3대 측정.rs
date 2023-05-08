use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let l: i32 = iter.next().unwrap().parse().unwrap();

    let mut count = 0;
    let mut vip: Vec<Vec<i32>> = vec![vec![0; 4]; n];

    for i in 0..n {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();

        vip[i][0] = iter.next().unwrap().parse().unwrap();
        vip[i][1] = iter.next().unwrap().parse().unwrap();
        vip[i][2] = iter.next().unwrap().parse().unwrap();
        vip[i][3] = if vip[i][0] >= l && vip[i][1] >= l && vip[i][2] >= l && vip[i][0] + vip[i][1] + vip[i][2] >= k {
            count += 1;
            1
        } else {
            0
        };
    }

    writeln!(writer, "{}", count).unwrap();
    for i in 0..n {
        if vip[i][3] == 1 {
            write!(writer, "{} {} {} ", vip[i][0], vip[i][1], vip[i][2]).unwrap();
        }
    }

    writer.flush().unwrap();
}
