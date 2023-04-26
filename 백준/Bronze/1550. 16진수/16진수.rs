use std::io;

// 16진수를 10진수로 변환하는 함수
fn hex_to_decimal(hex_num: &str) -> u64 {
    let hex_dict: Vec<(char, u64)> = vec![
        ('0', 0), ('1', 1), ('2', 2), ('3', 3),
        ('4', 4), ('5', 5), ('6', 6), ('7', 7),
        ('8', 8), ('9', 9), ('A', 10), ('B', 11),
        ('C', 12), ('D', 13), ('E', 14), ('F', 15),
    ];
    let mut decimal: u64 = 0;
    for (i, c) in hex_num.chars().rev().enumerate() {
        for (hex_char, hex_value) in &hex_dict {
            if c == *hex_char {
                decimal += hex_value * 16u64.pow(i as u32);
                break;
            }
        }
    }
    decimal
}

fn main() {
    // 입력 받기
    let mut hex_num = String::new();
    io::stdin().read_line(&mut hex_num)
        .expect("입력을 읽을 수 없습니다.");
    hex_num = hex_num.trim().to_uppercase();

    // 16진수를 10진수로 변환하여 출력
    let decimal = hex_to_decimal(&hex_num);
    println!("{}", decimal);
}
