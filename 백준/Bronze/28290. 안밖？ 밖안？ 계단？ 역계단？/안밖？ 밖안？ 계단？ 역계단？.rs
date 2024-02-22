use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n = input.trim();
    match n {
        "fdsajkl;" | "jkl;fdsa" => {writeln!(writer,"{}","in-out").unwrap()}
        "asdf;lkj" | ";lkjasdf" => {writeln!(writer,"{}","out-in").unwrap()}
        "asdfjkl;" => {writeln!(writer,"{}","stairs").unwrap()}
        ";lkjfdsa" => {writeln!(writer,"{}","reverse").unwrap()}
        _=>{writeln!(writer,"{}","molu").unwrap()}
    }
    writer.flush(). unwrap();
}
