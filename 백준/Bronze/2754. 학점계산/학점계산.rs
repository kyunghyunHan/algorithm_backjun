use std::io;

fn main() -> io::Result<()> {
    let mut arr = String::new();
    io::stdin().read_line(&mut arr)?;
    let str = arr.chars().nth(0).unwrap_or(' ');
    let left = arr.chars().nth(1).unwrap_or(' ');

    match str {
        'A' => {
            let grade = match left {
                '+' => 4.3,
                '0' => 4.0,
                '-' => 3.7,
                _ => 0.0,
            };
            println!("{:.1}\n", grade);
        }
        'B' => {
            let grade = match left {
                '+' => 3.3,
                '0' => 3.0,
                '-' => 2.7,
                _ => 0.0,
            };
            println!("{:.1}\n", grade);
        }
        'C' => {
            let grade = match left {
                '+' => 2.3,
                '0' => 2.0,
                '-' => 1.7,
                _ => 0.0,
            };
            println!("{:.1}\n", grade);
        }
        'D' => {
            let grade = match left {
                '+' => 1.3,
                '0' => 1.0,
                '-' => 0.7,
                _ => 0.0,
            };
            println!("{:.1}\n", grade);
        }
        _ => {
            println!("0.0\n");
        }
    }

    Ok(())
}
