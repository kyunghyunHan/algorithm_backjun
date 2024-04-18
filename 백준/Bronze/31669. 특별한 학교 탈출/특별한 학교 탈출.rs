use std::io::{BufReader, BufRead, BufWriter, Write, stdin, stdout};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let v: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut school_class = Vec::new();
    for _ in 0..v[0] {
        let mut row = String::new(); // Changed here: Declare row as mutable
        reader.read_line(&mut row).unwrap(); // Changed here: Use row instead of input
        let row_chars: Vec<char> = row.trim().chars().collect();
        school_class.push(row_chars);
    }
    let mut zip_class = vec![vec!['.'; v[0] as usize]; v[1] as usize];
    for i in 0..v[0] as usize {
        for j in 0..v[1] as usize {
            zip_class[j][i] = school_class[i][j];
        }
    }
    let mut result = 0;
    for (i, column) in zip_class.iter().enumerate() {
        if column.iter().filter(|&&c| c == 'X').count() == v[0] as usize {
            result = i + 1;
            break;
        }
    }

    if result > 0 {
        writeln!(writer, "{}", result).unwrap();
    } else {
        writeln!(writer, "{}", "ESCAPE FAILED").unwrap();
    }
}
