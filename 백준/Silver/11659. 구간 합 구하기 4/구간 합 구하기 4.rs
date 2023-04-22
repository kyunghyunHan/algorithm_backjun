use std::io::{stdin, stdout, BufRead, BufReader, Write, BufWriter};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let mut nums = buffer.trim().split_whitespace();
    let n = nums.next().unwrap().parse::<usize>().unwrap();
    let m = nums.next().unwrap().parse::<usize>().unwrap();

    let mut v: Vec<usize> = Vec::with_capacity(n);
    let mut buffer2 = String::new();
    reader.read_line(&mut buffer2).unwrap();
    let mut nums2 = buffer2.trim().split_whitespace();
    for _ in 0..n {
        let n = nums2.next().unwrap().parse::<usize>().unwrap();
        v.push(n);
    }

    let mut result_sum = vec![0; n];
    let mut sum = 0;
    for (i, &num) in v.iter().enumerate() {
        sum += num;
        result_sum[i] = sum;
    }

    for _ in 0..m {
        let mut buffer3 = String::new();
        reader.read_line(&mut buffer3).unwrap();
        let mut nums3 = buffer3.trim().split_whitespace();
        let n2 = nums3.next().unwrap().parse::<usize>().unwrap();
        let m2 = nums3.next().unwrap().parse::<usize>().unwrap();

        let result = if n2 == 1 {
            result_sum[m2 - 1]
        } else {
            result_sum[m2 - 1] - result_sum[n2 - 2]
        };
        writeln!(writer, "{}", result).unwrap();
    }
}
