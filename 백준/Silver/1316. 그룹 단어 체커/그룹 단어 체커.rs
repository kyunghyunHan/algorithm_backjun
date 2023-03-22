use std::io;

fn main() {
  
    let mut count = 0; // 그룹 단어가 아니면 카운트

    let mut n = String::new(); // 입력받을 단어의 개수
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    for _ in 0..n {
        let mut word = String::new(); // 입력받은 단어
        io::stdin().read_line(&mut word).unwrap();
        let word = word.trim();

        // 단어에서 알파벳 문자의 출현유무를 나타내는 배열 (출현없으면 false)
        let mut alphabet = [false; 26];
        alphabet[(word.chars().nth(0).unwrap() as u8 - 97) as usize] = true; // 첫번째 단어값을 true로 설정

        for i in 1..word.len() {
            // 1. i번째 문자가 i-1번째 문자와 같으면 연속이므로 넘어간다.
            if word.chars().nth(i).unwrap() == word.chars().nth(i - 1).unwrap() {
                continue;
            }
            // 2. i번째 문자가 i-1번째 문자와 같지 않고, (연속하지 않고)
            //    해당 배열값이 true라면 (이미 나왔던 문자라면)
            else if word.chars().nth(i).unwrap() != word.chars().nth(i - 1).unwrap() && alphabet[(word.chars().nth(i).unwrap() as u8 - 97) as usize] == true {
                count += 1; // 그룹단어가 아니므로 카운트
                break;
            }
            // 3. 위의 두 경우에 해당하지 않는 경우
            //    처음 등장한 문자인 경우
            else {
                alphabet[(word.chars().nth(i).unwrap() as u8 - 97) as usize] = true;
            }
        }

       
    }

    // 그룹 단어의 개수 = 전체단어의 개수 - 그룹단어가 아닌 단어의 개수
    println!("{}", n - count);
}