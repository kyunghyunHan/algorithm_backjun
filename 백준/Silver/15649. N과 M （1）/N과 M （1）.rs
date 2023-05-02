use std::io::{BufRead, BufReader, BufWriter, stdin, stdout, Write};

fn go(index: usize, n: usize, m: usize, visited: &mut [bool], ans: &mut [usize], writer: &mut BufWriter<std::io::StdoutLock>) {
    if index == m {
        writeln!(writer, "{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" ")).unwrap();
        return;
    }
    for i in 1..=n {
        if !visited[i] {
            visited[i] = true;
            ans[index] = i;
            go(index + 1, n, m, visited, ans, writer);
            visited[i] = false;
            ans[index] = 0;
        }
    }
}

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut visited = vec![false; n + 1];
    let mut ans = vec![0; m];
    go(0, n, m, &mut visited, &mut ans, &mut writer);
    writer.flush().unwrap();
}
