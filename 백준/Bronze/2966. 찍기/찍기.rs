use std::io::{BufRead,BufReader,BufWriter,Write,stdin,stdout};

fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut a = ['A', 'B', 'C'];
    let mut b = ['B', 'A', 'B', 'C'];
    let mut c = ['C', 'C', 'A', 'A', 'B', 'B'];

    let mut cnt = [0, 0, 0];
  
    let mut result = 0;
    let mut input = String::new();
    reader
        .read_line(&mut input)
        .unwrap();

   let mut  n = input.trim().parse().unwrap();
    input.clear();
    reader.
        read_line(&mut input).unwrap();

    let arr = input.trim().as_bytes();

    for i in 0..n {
        if a[i % 3] == arr[i] as char {
            cnt[0] += 1;
        }
        if b[i % 4] == arr[i] as char {
            cnt[1] += 1;
        }
        if c[i % 6] == arr[i] as char {
            cnt[2] += 1;
        }
    }

    for i in 0..3 {
        if result < cnt[i] {
            result = cnt[i];
        }
    }

    println!("{}", result);

    if cnt[0] == result {
        println!("Adrian");
    }
    if cnt[1] == result {
        println!("Bruno");
    }
    if cnt[2] == result {
        println!("Goran");
    }
}
