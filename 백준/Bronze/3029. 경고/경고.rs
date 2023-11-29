use std::io;

fn main() {
    let mut input = String::new();

    // 입력 받기
    io::stdin().read_line(&mut input).unwrap();
    let mut times1 = input.trim().split(':').map(|x| x.parse::<i32>().unwrap());

    let mut input = String::new();
    ;
    io::stdin().read_line(&mut input).unwrap();
    let mut times2 = input.trim().split(':').map(|x| x.parse::<i32>().unwrap());

    // 시간 계산
    let t1 = times1.next().unwrap() * 60 * 60 + times1.next().unwrap() * 60 + times1.next().unwrap();
    let t2 = times2.next().unwrap() * 60 * 60 + times2.next().unwrap() * 60 + times2.next().unwrap();
    let t = if t2 > t1 { t2 - t1 } else { t2 - t1 + 24 * 60 * 60 };

    let h = t / 60 / 60;
    let m = t / 60 % 60;
    let s = t % 60;

    println!("{:02}:{:02}:{:02}", h, m, s);
}
