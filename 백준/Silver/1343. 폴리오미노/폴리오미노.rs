use std::io;

fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let mut cnt = 0;
    // let mut result: Vec<&str> = Vec::new();
    let mut result = "".to_string();
    for i in 0..input_a.len() {
        if &input_a[i..i + 1] == "X" {
            cnt = cnt + 1;
        }
        if &input_a[i..i + 1] == "." {
            // result.push(".");
            result = result + ".";
            if (cnt % 2) != 0 {
                break;
            } else {
                cnt = 0;
            }
        }
        if (cnt == 2) && (&input_a[i + 1..i + 2]) != "X" {
            // result.push("BB");
            result = result + "BB";
            cnt = 0;
        } else if cnt == 4 {
            result = result + "AAAA";
            // result.push("AAAA");
            cnt = 0;
        }
    }
    if (cnt % 2) == 1 {
        println!("{}", -1);
    } else {
        println!("{}", result.as_str());
    }
}
