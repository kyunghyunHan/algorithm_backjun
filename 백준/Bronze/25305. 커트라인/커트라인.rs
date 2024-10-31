use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn sort(arr: &mut Vec<i64>, n: usize) {
    for i in 1..n {
        let key = arr[i];
        let mut j = i as isize - 1;
        while j >= 0 && arr[j as usize] < key {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    }
}

fn main() {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();
    
    if let Some(Ok(line)) = input.next() {
        let nk = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();
        let n = nk[0] as usize;
        let k = nk[1] as usize;
        
        if let Some(Ok(line)) = input.next() {
            let mut arr = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i64>>();
            sort(&mut arr, n);
            writeln!(writer, "{}", arr[k - 1]).unwrap();
        } else {
            writeln!(writer, "Input not defined").unwrap();
        }
    } else {
        writeln!(writer, "Input not defined").unwrap();
    }
    
    writer.flush().unwrap();
}
