use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let mut sizes = line.split_whitespace().map(|s| s.parse().unwrap());
    let n = sizes.next().unwrap();
    let m = sizes.next().unwrap();

    let mut matrix_a = vec![vec![0; m as usize]; n as usize];
    for i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let mut elements = line.split_whitespace().map(|s| s.parse().unwrap());
        for j in 0..m {
            matrix_a[i as usize][j as usize] = elements.next().unwrap();
        }
    }

    let mut matrix_b = vec![vec![0; m as usize]; n as usize];
    for i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let mut elements = line.split_whitespace().map(|s| s.parse().unwrap());
        for j in 0..m {
            matrix_b[i as usize][j as usize] = elements.next().unwrap();
        }
    }

    let mut matrix_c = vec![vec![0; m as usize]; n as usize];
    for i in 0..n {
        for j in 0..m {
            matrix_c[i as usize][j as usize] = matrix_a[i as usize][j as usize] + matrix_b[i as usize][j as usize];
        }
    }

    for row in matrix_c {
        for element in row {
            print!("{} ", element);
        }
        println!();
    }
}