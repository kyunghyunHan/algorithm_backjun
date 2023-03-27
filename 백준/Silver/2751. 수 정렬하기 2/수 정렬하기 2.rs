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
    let cnt: usize = buffer.trim().parse().unwrap();

    let mut arr: Vec<i32> = Vec::with_capacity(cnt);
    let mut buf: Vec<i32> = Vec::with_capacity(cnt);

    for _ in 0..cnt {
        buffer.clear();
        reader.read_line(&mut buffer).unwrap();
        arr.push(buffer.trim().parse().unwrap());
        buf.push(0);
    }

    merge_sort(&mut arr, &mut buf, 0, cnt - 1);

    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for x in arr.iter() {
        writeln!(writer, "{}", x).unwrap();
    }
}

