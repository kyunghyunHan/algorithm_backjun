use std::io::{BufReader,BufWriter,BufRead,Write,stdin,stdout};


fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input ).unwrap();

    let mut nm = input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = nm.next().unwrap();
    let m = nm.next().unwrap();

    let mut tree = Vec::new();
   
   input.clear();
   reader.read_line(&mut input ).unwrap();
   let mut nums= input.trim().split_whitespace().map(|x|x.parse::<usize>().unwrap());


    for x in 0..n {
     let x= nums.next().unwrap();
        tree.push(x);
    }

    let mut start = 0;
    let mut end = *tree.iter().max().unwrap();
    let mut result = 0;

    while start <= end {
        let mut total: u64 = 0;
        let mid = (start + end) / 2;

        for x in &tree {
            if *x > mid {
                total += (*x as u64) - (mid as u64);
            }
        }

        if total < m as u64 {
            end = mid - 1;
        } else {
            result = mid;
            start = mid + 1;
        }
    }

    println!("{}", result);
}
