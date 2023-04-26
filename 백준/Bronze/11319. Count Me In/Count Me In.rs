use std::io;

fn main() {
    let vowels = vec!['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("입력을 읽을 수 없습니다.");
    let n: usize = n.trim().parse().expect("유효한 숫자를 입력하세요.");

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("입력을 읽을 수 없습니다.");

        let mut c = 0;
        let mut v = 0;
        for t in s.chars() {
            if vowels.contains(&t) {
                v += 1;
            } else if t.is_alphabetic() {
                c += 1;
            }
        }
        println!("{} {}", c, v);
    }
}
