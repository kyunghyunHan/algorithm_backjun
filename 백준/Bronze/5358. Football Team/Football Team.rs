use std::io::{self, BufRead};

fn football_team(name: &str) -> String {
    let change_word_dict: [(char, char); 4] = [('e', 'i'), ('i', 'e'), ('E', 'I'), ('I', 'E')];
    
    let new_name: String = name.chars()
        .map(|c| {
            change_word_dict.iter()
                .find(|&(from, _)| *from == c)
                .map(|&(_, to)| to)
                .unwrap_or(c)
        })
        .collect();
    
    new_name
}

fn main() {
    loop {
        let mut name = String::new();
        match io::stdin().lock().read_line(&mut name) {
            Ok(0) => break,
            Ok(_) => {
                name = name.trim().to_string();
                println!("{}", football_team(&name));
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}
