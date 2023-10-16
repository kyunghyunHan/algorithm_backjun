use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
   let mut reader= BufReader::new(stdin().lock());
   let mut writer= BufWriter::new(stdout().lock());
   let mut input = String::new();
   let mut day= 0;
   let mut month=vec![31,28,31,30,31,30,31,31,30,31,30,31];
   let week = vec!["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
   reader.read_line(&mut input).unwrap();


   let mut input_iter = input.split_whitespace();
   let month: i32 = input_iter.next().expect("Missing month").parse().expect("Invalid month");
   let day: i32 = input_iter.next().expect("Missing day").parse().expect("Invalid day");
  let days_of_month: [i32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
   let day_of_week: [&str; 7] = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];

   let mut day_count = day;

   for i in 1..month {
      day_count += days_of_month[i as usize];
  }
   let day_index = day_count % 7;

   writeln!(writer,"{}",day_of_week[day_index as usize]).unwrap();
}