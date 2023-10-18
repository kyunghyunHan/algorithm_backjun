
use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
fn main(){
   let mut reader= BufReader::new(stdin().lock());
   let mut writer= BufWriter::new(stdout().lock());
   let mut input = String::new();
   let mut histogram = Vec::new();
   reader.read_line(&mut input).unwrap();

   let n: i32 = input.trim().parse().unwrap();

   input.clear();
   reader.read_line(&mut input).unwrap();

   let str_values: Vec<&str> = input.trim().split_whitespace().collect();

   for str_value in str_values {
      let value: i32 = str_value.parse().unwrap();
      histogram.push(value);
  }

  let max_area = get_max_area(&histogram);
   writeln!(writer,"{}",max_area).unwrap();
   writer.flush().unwrap();
}

fn get_max_area(histogram: &Vec<i32>) -> i64 {
   let mut stack: Vec<usize> = Vec::new();
   let mut max_area: i64 = 0;

   for i in 0..histogram.len() {
       while !stack.is_empty() && histogram[stack[stack.len() - 1]] > histogram[i] {
           let height = histogram[stack.pop().unwrap()];
           let width = if stack.is_empty() {
               i as i32
           } else {
               (i - stack[stack.len() - 1] - 1) as i32
           };
           max_area = max_area.max((width as i64) * (height as i64));
       }
       stack.push(i);
   }

   while !stack.is_empty() {
       let height = histogram[stack.pop().unwrap()];
       let width = if stack.is_empty() {
           histogram.len() as i32
       } else {
           (histogram.len() - stack[stack.len() - 1] - 1) as i32
       };
       max_area = max_area.max((width as i64) * (height as i64));
   }

   max_area
}