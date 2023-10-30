use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n:i32= input.trim().parse().unwrap();
    // 카운팅 배열 초기화
    let mut arr: Vec<i32> = Vec::with_capacity(n as usize);
    let mut buf: Vec<i32> = Vec::with_capacity(n as usize);
    //배열에 삽입
    for _ in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        arr.push(input.trim().parse().unwrap());
        buf.push(0);
    }
    merge_sort(&mut arr, &mut buf, 0, n as usize - 1);
    for x in arr.iter() {
        writeln!(writer, "{}", x).unwrap();
    }
    writer.flush().unwrap();
}

//합병정렬
//배열 a를 low와 high 사이에서 합병 정렬
//배열을 반으로 나누어 정렬하고 그 후에 정렬된 하위 배열을 합병하여 전체 배열을 정렬
fn merge_sort(a: &mut [i32], b: &mut [i32], low: usize, high: usize) {
    //high - low가 1보다 작다면, 즉, 배열의 길이가 1이하면 이미 정렬된 상태이므로 함수를 종료
    if high - low < 1 {
        return;
    }

    let mid = (low + high) / 2;
    //merge_sort를 재귀적으로 호출하여 각각을 정렬
    merge_sort(a, b, low, mid);
    merge_sort(a, b, mid + 1, high);

    let mut i = low;
    let mut j = mid + 1;
    let mut k = low;
    /*분할 정렬된 list의 합병 */
    //i와 j는 각 하위 배열을 순회하기 위한 인덱스를 나타냄
    //k는 보조 배열 b의 인덱스를 나타냄
    //두 하위 배열을 하나의 정렬된 배열로 합치는데, 두 배열의 원소를 비교하여 작은 원소를 b에 복사
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
    //첫 번째 하위 배열의 요소들을 b 배열로 복사
    // i는 첫 번째 하위 배열을 나타내는 인덱스
    while i <= mid {
        b[k] = a[i];
        i += 1;
        k += 1;
    }
  //두 번째 하위 배열의 요소들을 b 배열로 복사
  //j는 두 번째 하위 배열을 나타내는 인덱스
    while j <= high {
        b[k] = a[j];
        j += 1;
        k += 1;
    }
     //배열
     //a 배열의 low부터 high까지의 범위를 b 배열의 값으로 업데이트하여 정렬된 배열
    for i in low..=high {
        a[i] = b[i];
    }
}