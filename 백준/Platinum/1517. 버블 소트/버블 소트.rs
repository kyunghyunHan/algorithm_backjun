use std::io;

fn merge(left: usize, mid: usize, right: usize, arr: &mut Vec<i64>, sorted: &mut Vec<i64>, ans: &mut u64) {
    let mut i = left;
    let mut j = mid + 1;
    let mut k = left;
    while i <= mid && j <= right {
        if arr[i] <= arr[j] {
            sorted[k] = arr[i];
            i += 1;
        } else {
            sorted[k] = arr[j];
            *ans += (j - k) as u64;
            j += 1;
        }
        k += 1;
    }
    if i > mid {
        for x in j..=right {
            sorted[k] = arr[x];
            k += 1;
        }
    } else {
        for x in i..=mid {
            sorted[k] = arr[x];
            k += 1;
        }
    }
    for x in left..=right {
        arr[x] = sorted[x];
    }
}

fn merge_sort(left: usize, right: usize, arr: &mut Vec<i64>, sorted: &mut Vec<i64>, ans: &mut u64) {
    if left >= right {
        return;
    }
    let mid = (left + right) >> 1;
    merge_sort(left, mid, arr, sorted, ans);
    merge_sort(mid + 1, right, arr, sorted, ans);
    merge(left, mid, right, arr, sorted, ans);
}

fn main() {
    let mut arr: Vec<i64> = Vec::new();
    let mut sorted: Vec<i64> = vec![0; 500001];
    let mut ans: u64 = 0;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Failed to parse input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse input"))
        .collect();

    arr.extend_from_slice(&nums[..n]);

    merge_sort(0, n - 1, &mut arr, &mut sorted, &mut ans);

    println!("{}", ans);
}