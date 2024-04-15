use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid integer");

    let mut result = 0;

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let skateboard: Vec<i32> = input.split_whitespace()
                                       .map(|s| s.parse().expect("Please enter valid integers"))
                                       .collect();
        let (run, trick) = skateboard.split_at(2);
        let mut run_vec: Vec<i32> = run.to_vec();
        let mut trick_vec: Vec<i32> = trick.to_vec();

        run_vec.sort_by(|a, b| b.cmp(a));
        trick_vec.sort_by(|a, b| b.cmp(a));

        result = result.max(run_vec[0] + trick_vec.iter().take(2).sum::<i32>());
    }

    println!("{}", result);
}
