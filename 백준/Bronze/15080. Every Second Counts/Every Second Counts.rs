use std::io;

fn main() {
    let mut a = [0; 3];
    let mut b = [0; 3];
    let mut start_time = 0;
    let mut end_time = 0;

    // Read input for 'a'
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split(':').collect();
    for (i, part) in parts.iter().enumerate() {
        a[i] = part.trim().parse().expect("Failed to parse input");
    }

    // Read input for 'b'
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split(':').collect();
    for (i, part) in parts.iter().enumerate() {
        b[i] = part.trim().parse().expect("Failed to parse input");
    }

    // Calculate start time and end time in seconds
    start_time = a[2] + a[1] * 60 + a[0] * 3600;
    end_time = b[2] + b[1] * 60 + b[0] * 3600;

    // Calculate the time difference
    let diff = if start_time > end_time {
        24 * 3600 - start_time + end_time
    } else {
        end_time - start_time
    };

    println!("{}", diff);
}
