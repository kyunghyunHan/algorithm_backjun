use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

fn main() {
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let  n= input.trim().parse::<i32>().unwrap();

    for i in 1..=n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let g:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse::<i32>().unwrap()).collect();
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s:Vec<i32>= input.trim().split_whitespace().map(|x|x.parse::<i32>().unwrap()).collect();
        let g_score = g[0] + g[1] * 2 + (g[2] + g[3]) * 3 + g[4] * 4 + g[5] * 10;
        let s_score = s[0] + (s[1] + s[2] + s[3]) * 2 + s[4] * 3 + s[5] * 5 + s[6] * 10;
        if g_score > s_score {
            writeln!(writer,"Battle {}: Good triumphs over Evil", i).unwrap();
        } else if g_score < s_score {
            writeln!(writer,"Battle {}: Evil eradicates all trace of Good", i).unwrap();
        } else {
            writeln!(writer,"Battle {}: No victor on this battle field", i).unwrap();
        }
    }
    writer.flush().unwrap();
}
