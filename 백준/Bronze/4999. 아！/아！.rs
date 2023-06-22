use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(jaeah)) = lines.next() {
        if let Some(Ok(doctor)) = lines.next() {
            if jaeah.len() >= doctor.len() {
                println!("go");
            } else {
                println!("no");
            }
        }
    }
}