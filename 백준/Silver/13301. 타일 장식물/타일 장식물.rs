use std::io::{BufReader,BufWriter,Write,stdin,stdout,BufRead};
fn main(){
    let mut reader = BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    match input.next(){
        Some(Ok(line))=>{
           let n =  line.parse::<usize>().unwrap();

           let mut dp =[0;80];

           dp[1]= 2;
           dp[2] =3;
           for i in 3..=n{
              dp[i] = (dp[i-1]+dp[i-2])
           } 
           writeln!(writer,"{}",dp[n]*2).unwrap();
        }
        _=>{}
    }
    writer.flush().unwrap();
}


  //둘레 2 3 5 8 13 21
  //    1 1 2 3 5  8 