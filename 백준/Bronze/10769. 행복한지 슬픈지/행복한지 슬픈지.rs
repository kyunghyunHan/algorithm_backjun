use std::io;

fn main() {
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read input");

    let happy = ":-)";
    let sad = ":-(";
    let mut happy_n = 0;
    let mut sad_n = 0;

    for i in 0..sentence.len() - 2 {
        let temp = &sentence[i..i + 3];
        match temp {
            x if x == happy => happy_n += 1,
            x if x == sad => sad_n += 1,
            _ => (),
        }
    }

    if happy_n == 0 && sad_n == 0 {
        println!("none");
    } else if happy_n == sad_n {
        println!("unsure");
    } else if happy_n > sad_n {
        println!("happy");
    } else {
        println!("sad");
    }
}
