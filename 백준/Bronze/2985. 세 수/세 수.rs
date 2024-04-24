use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let mut nums = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let a = nums.next().unwrap();
    let b = nums.next().unwrap();
    let c = nums.next().unwrap();

    if a + b == c {
        println!("{}+{}={}", a, b, c);
    } else if a - b == c {
        println!("{}-{}={}", a, b, c);
    } else if a * b == c {
        println!("{}*{}={}", a, b, c);
    } else if a / b == c {
        println!("{}/{}={}", a, b, c);
    } else if a == b + c {
        println!("{}={}+{}", a, b, c);
    } else if a == b - c {
        println!("{}={}-{}", a, b, c);
    } else if a == b * c {
        println!("{}={}*{}", a, b, c);
    } else if a == b / c {
        println!("{}={}/{}", a, b, c);
    }
}
