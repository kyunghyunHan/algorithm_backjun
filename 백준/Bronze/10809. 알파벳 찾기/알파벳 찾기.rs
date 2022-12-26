use std::io;
fn main() {
    let alpabat = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let s: Vec<&str> = input_a.split_whitespace().collect();
    let mut arr: Vec<i32> = vec![-1; 26];
    let mut s_arr: Vec<&str> = vec![];

    for i in 0..s[0].bytes().len() {
        let s = &s[0][i..i + 1];
        s_arr.push(s);
    }
    for i in 0..s_arr.len() {
        let mut test = s_arr[i].as_bytes()[0];
        if arr[test as usize - 97 as usize] == -1 {
            arr[test as usize - 97 as usize] = i as i32;
        }
    }

    // for i in 0..alpabat.len() {
    //     for j in 0..s_arr.len() {
    //         if alpabat[i] == s_arr[j] {
    //             arr[i] = j as i32
    //         }
    //     }
    // }

    println!(
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        arr[0],
        arr[1],
        arr[2],
        arr[3],
        arr[4],
        arr[5],
        arr[6],
        arr[7],
        arr[8],
        arr[9],
        arr[10],
        arr[11],
        arr[12],
        arr[13],
        arr[14],
        arr[15],
        arr[16],
        arr[17],
        arr[18],
        arr[19],
        arr[20],
        arr[21],
        arr[22],
        arr[23],
        arr[24],
        arr[25],
    )
}
