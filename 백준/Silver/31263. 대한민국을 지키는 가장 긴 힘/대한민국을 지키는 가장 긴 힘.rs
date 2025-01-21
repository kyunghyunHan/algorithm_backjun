use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();
        let mut v: Vec<usize> = Vec::new();

        if let Some(Ok(line)) = input.next() {
            let mut ans = 0; // 병사 수

            let mut lst: Vec<char> = line.trim().chars().collect();
            while !lst.is_empty() {
                if lst.len() <= 3 {
                    let num: String = lst.iter().take(3).collect();
                    if num.parse::<usize>().unwrap() <= 641 {
                        ans += 1;
                        break;
                    } else {
                        lst.drain(0..2); // 첫 두 문자를 제거
                        ans += 1;
                    }
                } else {
                    if lst[2] == '0' {
                        if lst.len() > 3 && lst[3] == '0' {
                            lst.drain(0..1); // 첫 번째 문자를 제거
                            ans += 1;
                        } else {
                            let num: String = lst.iter().take(3).collect();
                            if num.parse::<usize>().unwrap() <= 641 {
                                lst.drain(0..3); // 첫 세 문자를 제거
                                ans += 1;
                            } else {
                                lst.drain(0..1); // 첫 번째 문자를 제거
                                ans += 1;
                            }
                        }
                    } else {
                        if lst.len() > 3 && lst[3] == '0' {
                            lst.drain(0..2); // 첫 두 문자를 제거
                            ans += 1;
                        } else {
                            let num: String = lst.iter().take(3).collect();
                            if num.parse::<usize>().unwrap() <= 641 {
                                lst.drain(0..3); // 첫 세 문자를 제거
                                ans += 1;
                            } else {
                                lst.drain(0..2); // 첫 두 문자를 제거
                                ans += 1;
                            }
                        }
                    }
                }
            }
            writeln!(writer, "{}", ans).unwrap();
        }
    }
}
