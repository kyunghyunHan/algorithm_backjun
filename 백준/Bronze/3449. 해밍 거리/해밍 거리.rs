use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    let t: usize = input.next().unwrap().unwrap().trim().parse().unwrap(); // 테스트 케이스 수

    for _ in 0..t {
        let bin1 = input.next().unwrap().unwrap(); // 첫 번째 이진수
        let bin2 = input.next().unwrap().unwrap(); // 두 번째 이진수

        let hamming_distance = bin1
            .chars()
            .zip(bin2.chars())
            .filter(|(a, b)| a != b)
            .count();

        writeln!(writer, "Hamming distance is {}.", hamming_distance).unwrap()
    }
    writer.flush().unwrap();
}
