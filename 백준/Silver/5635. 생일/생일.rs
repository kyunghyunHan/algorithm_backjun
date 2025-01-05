use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Stdout, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let n = input.trim().parse::<i32>().unwrap();

    let mut students: Vec<(String, (i32, i32, i32))> = Vec::new();

    for i in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut v = input
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let name = v[0].to_string();
        let day: i32 = v[1].parse().unwrap();
        let month: i32 = v[2].parse().unwrap();
        let year: i32 = v[3].parse().unwrap();
        students.push((name, (year, month, day)));
    }
    let youngest = students
        .iter()
        .max_by(|a, b| compare_dates(&a.1, &b.1))
        .unwrap();
    let oldest = students
        .iter()
        .min_by(|a, b| compare_dates(&a.1, &b.1))
        .unwrap();
    writeln!(writer, "{}", youngest.0).unwrap();
    writeln!(writer, "{}", oldest.0).unwrap();
    writer.flush().unwrap();
}
fn compare_dates(date1: &(i32, i32, i32), date2: &(i32, i32, i32)) -> std::cmp::Ordering {
    if date1.0 != date2.0 {
        date1.0.cmp(&date2.0) // 연도 비교
    } else if date1.1 != date2.1 {
        date1.1.cmp(&date2.1) // 월 비교
    } else {
        date1.2.cmp(&date2.2) // 일 비교
    }
}
