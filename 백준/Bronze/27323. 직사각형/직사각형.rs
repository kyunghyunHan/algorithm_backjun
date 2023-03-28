use std::io::{stdin, BufRead, BufReader, BufWriter, Write};

fn merge_sort(a: &mut [i32], b: &mut [i32], low: usize, high: usize) {
    if high - low < 1 {
        return;
    }

    let mid = (low + high) / 2;

    merge_sort(a, b, low, mid);
    merge_sort(a, b, mid + 1, high);

    let mut i = low;
    let mut j = mid + 1;
    let mut k = low;

    while i <= mid && j <= high {
        if a[i] <= a[j] {
            b[k] = a[i];
            i += 1;
        } else {
            b[k] = a[j];
            j += 1;
        }
        k += 1;
    }

    while i <= mid {
        b[k] = a[i];
        i += 1;
        k += 1;
    }

    while j <= high {
        b[k] = a[j];
        j += 1;
        k += 1;
    }

    for i in low..=high {
        a[i] = b[i];
    }
}

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let mut buffer2 = String::new();
    reader.read_line(&mut buffer2).unwrap();
    let cnt: usize = buffer.trim().parse().unwrap();

    let cnt2: usize = buffer2.trim().parse().unwrap(); 

   println!("{}",cnt*cnt2)
}
