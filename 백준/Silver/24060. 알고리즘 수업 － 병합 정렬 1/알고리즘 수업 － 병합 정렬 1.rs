use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn merge_sort(arr: &mut [usize], start: usize, end: usize, k: usize, input_cnt: &mut usize) {
    if start < end {
        let mid = (start + end) / 2;
        merge_sort(arr, start, mid, k, input_cnt);
        merge_sort(arr, mid + 1, end, k, input_cnt);
        merge(arr, start, mid, end, k, input_cnt);
    }
}

fn merge(arr: &mut [usize], p: usize, q: usize, r: usize, k: usize, input_cnt: &mut usize) {
    let mut tmp = Vec::with_capacity(r - p + 1);
    let mut i = p;
    let mut j = q + 1;
    let mut t = 0;

    while i <= q && j <= r {
        if arr[i] <= arr[j] {
            tmp.push(arr[i]);
            i += 1;
        } else {
            tmp.push(arr[j]);
            j += 1;
        }
        t += 1;
    }

    while i <= q {
        tmp.push(arr[i]);
        i += 1;
        t += 1;
    }

    while j <= r {
        tmp.push(arr[j]);
        j += 1;
        t += 1;
    }

    let mut i = p;
    let mut t = 0;

    while i <= r {
        arr[i] = tmp[t];
        i += 1;
        t += 1;
        *input_cnt += 1;
        if *input_cnt == k {
            print!("{}\n", tmp[t - 1]);
            io::stdout().flush().unwrap();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut inputs = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = inputs.next().unwrap();
    let k = inputs.next().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();
    let nums = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut arr: Vec<usize> = Vec::with_capacity(n);
    for i in 0..n {
        arr.push(nums[i]);
    }

    let mut input_cnt = 0;
    merge_sort(&mut arr, 0, n - 1, k, &mut input_cnt);
    if input_cnt < k {
        println!("-1");
    }
}
