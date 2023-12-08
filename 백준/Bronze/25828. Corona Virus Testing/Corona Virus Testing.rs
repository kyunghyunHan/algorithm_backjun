use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let values: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.trim().parse().expect("Expected a number"))
        .collect();
    
    let g = values[0]; // Number of groups
    let p = values[1]; // Number of people in each group
    let t = values[2]; // Number of groups tested positive
    
    let total_individual = g * p; // Total number of individuals
    
    // Calculate the number of test kits required for individual and group testing
    let kits_individual = total_individual;
    let kits_groups = g + (t * p);
    
    // Determine which approach uses fewer test kits
    let result = if kits_individual < kits_groups {
        1 // Individual testing requires fewer kits
    } else if kits_groups < kits_individual {
        2 // Group testing requires fewer kits
    } else {
        0 // Both approaches use the same number of kits
    };
    
    println!("{}", result);
}
