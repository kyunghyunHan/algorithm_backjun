use std::io;
fn main() {
    let mut v: Vec<i32> = Vec::new();
    for i in 0..10 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let inputs: Vec<&str> = input.split_whitespace().collect();
        let mut number_first: i32 = inputs[0].parse::<i32>().unwrap();
        let result2 = number_first % 42;
        v.push(result2);
    }
    let mut result = 0;

    for i in 0..10 {
        let mut count = 0;
        for j in 0..i {
            if v[i] == v[j] {
                count += 1;
            }
        }
        if count == 0 {
            result += 1;
        }
    }
    println!("{}", result)
}
//나머지
