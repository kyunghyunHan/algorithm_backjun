use std::collections::HashSet;
use std::io;

fn main() {
    let mut croatia = String::new();
    io::stdin()
        .read_line(&mut croatia)
        .expect("Failed to read input");
    croatia = croatia.trim().to_string();

    let word_map: HashSet<&str> = ["c=", "c-", "d-", "lj", "nj", "s=", "z="]
        .iter()
        .cloned()
        .collect();

    let mut total = 1;
    for i in 1..croatia.len() {
        let mut temp: String = croatia[i - 1..i + 1].into();
        if word_map.contains(&*temp) {
            if temp == "z=" && i > 1 {
                let temp: String = croatia[i - 2..i + 1].into();
                if temp == "dz=" {
                    total -= 1;
                }
            }

            continue;
        }

        total += 1;
    }

    println!("{}", total);
}
