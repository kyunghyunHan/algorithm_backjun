use std::io;

fn main() {
    let mut c: [[i32; 5]; 1000] = [[0; 5]; 1000]; 
    let mut score: [i32; 1000] = [0; 1000];
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input"); // Number of people input
    
    let input: usize = input.trim().parse().expect("Invalid input");
    
    for i in 0..input {
        let mut card_values = String::new();
        io::stdin().read_line(&mut card_values).expect("Failed to read card values");
        
        let card_values: Vec<i32> = card_values
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid card value"))
            .collect();
        
        for j in 0..5 {
            c[i][j] = card_values[j];
        }
    }
    
    let mut max = 0;
    let mut num = 0; 
    
    for i in 0..input {
        for j in 0..5 {
            for k in (j + 1)..5 {
                for l in (k + 1)..5 {
                    let sum = (c[i][j] + c[i][k] + c[i][l]) % 10;
                    if score[i] < sum {
                        score[i] = sum;
                    }
                }
            }
        }
    }
    
    for i in 0..input {
        if max < score[i] {
            max = score[i];
            num = i;
        }
        
        if max == score[i] && i > num {
            num = i;
        }
    }
    
    println!("{}", num + 1);
}
