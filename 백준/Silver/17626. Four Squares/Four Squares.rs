use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer=BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n= input.trim().parse().unwrap();

 
    let mut dp: Vec<usize> = vec![0, 1];

    for i in 2..=n {
        /*10의 9승 */
        let mut min_value = 1e9 as usize;
        let mut j = 1;

        while j * j <= i {
            /*비교 */
            min_value = min_value.min(dp[i - j * j]);
            j += 1;
        }

        dp.push(min_value + 1);
    }
    writeln!(writer,"{}",dp[n]).unwrap();
    writer.flush().unwrap();

}