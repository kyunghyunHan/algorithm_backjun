use std::io;
//두번째 숫자-첫번쨰 숫자= 세번째 숫자-두번쨰 숫자->등차수열
//두번쨰ㅉ 숫자/첫번째숫자 = 세번쨰 숫자/두번재 숫자->등비수열

fn main() {
    let mut li: Vec<i32> = Vec::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let num: i32 = input.trim().parse().unwrap();
        li.push(num);
    }
    let ans = li[n - 1];

    //등차수열인지
    if li[2] - li[1] == li[1] - li[0] {
        let common_diff = li[1] - li[0];
        let mut next_num = li[2] + common_diff;
        for i in 3..n {
            if li[i] != next_num {
                println!("{}", ans);
                return;
            }
            next_num += common_diff;
        }
        println!("{}", li[n - 1] + common_diff);
    //등비수열인지
    } else if li[2] / li[1] == li[1] / li[0] {
        let common_ratio = li[1] / li[0];
        let mut next_num = li[2] * common_ratio;
        for i in 3..n {
            if li[i] != next_num {
                println!("{}", ans);
                return;
            }
            next_num *= common_ratio;
        }
        println!("{}", li[n - 1] * common_ratio);
    } else {
        println!("{}", ans);
    }
}
